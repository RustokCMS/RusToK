use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};
use std::collections::HashSet;
use tracing::{info, instrument};
use uuid::Uuid;

use rustok_core::{generate_id, DomainEvent, EventBus};

use crate::dto::*;
use crate::entities;
use crate::error::{CommerceError, CommerceResult};

pub struct CatalogService {
    db: DatabaseConnection,
    event_bus: EventBus,
}

impl CatalogService {
    pub fn new(db: DatabaseConnection, event_bus: EventBus) -> Self {
        Self { db, event_bus }
    }

    #[instrument(skip(self, input), fields(tenant_id = %tenant_id))]
    pub async fn create_product(
        &self,
        tenant_id: Uuid,
        actor_id: Uuid,
        input: CreateProductInput,
    ) -> CommerceResult<ProductResponse> {
        if input.translations.is_empty() {
            return Err(CommerceError::Validation(
                "At least one translation is required".into(),
            ));
        }
        if input.variants.is_empty() {
            return Err(CommerceError::NoVariants);
        }

        let product_id = generate_id();
        let now = Utc::now();

        let txn = self.db.begin().await?;

        let product = entities::product::ActiveModel {
            id: Set(product_id),
            tenant_id: Set(tenant_id),
            status: Set(if input.publish {
                "active".into()
            } else {
                "draft".into()
            }),
            vendor: Set(input.vendor.clone()),
            product_type: Set(input.product_type.clone()),
            metadata: Set(input.metadata.clone()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            published_at: Set(if input.publish {
                Some(now.into())
            } else {
                None
            }),
        };
        product.insert(&txn).await?;

        let mut seen = HashSet::new();
        for trans_input in &input.translations {
            let handle = trans_input
                .handle
                .clone()
                .unwrap_or_else(|| Self::slugify(&trans_input.title));

            let key = format!("{}::{}", trans_input.locale, handle.clone());
            if !seen.insert(key) {
                return Err(CommerceError::DuplicateHandle {
                    handle,
                    locale: trans_input.locale.clone(),
                });
            }

            let existing = entities::product_translation::Entity::find()
                .filter(entities::product_translation::Column::Locale.eq(&trans_input.locale))
                .filter(entities::product_translation::Column::Handle.eq(&handle))
                .one(&txn)
                .await?;
            if existing.is_some() {
                return Err(CommerceError::DuplicateHandle {
                    handle,
                    locale: trans_input.locale.clone(),
                });
            }

            let translation = entities::product_translation::ActiveModel {
                id: Set(generate_id()),
                product_id: Set(product_id),
                locale: Set(trans_input.locale.clone()),
                title: Set(trans_input.title.clone()),
                handle: Set(handle),
                description: Set(trans_input.description.clone()),
                meta_title: Set(trans_input.meta_title.clone()),
                meta_description: Set(trans_input.meta_description.clone()),
            };
            translation.insert(&txn).await?;
        }

        for (position, opt_input) in input.options.iter().enumerate() {
            let option = entities::product_option::ActiveModel {
                id: Set(generate_id()),
                product_id: Set(product_id),
                position: Set(position as i32),
                name: Set(opt_input.name.clone()),
                values: Set(serde_json::to_value(&opt_input.values)
                    .map_err(|error| CommerceError::Validation(error.to_string()))?),
            };
            option.insert(&txn).await?;
        }

        for (position, var_input) in input.variants.iter().enumerate() {
            let variant_id = generate_id();

            if let Some(ref sku) = var_input.sku {
                let existing = entities::product_variant::Entity::find()
                    .filter(entities::product_variant::Column::TenantId.eq(tenant_id))
                    .filter(entities::product_variant::Column::Sku.eq(sku))
                    .one(&txn)
                    .await?;
                if existing.is_some() {
                    return Err(CommerceError::DuplicateSku(sku.clone()));
                }
            }

            let variant = entities::product_variant::ActiveModel {
                id: Set(variant_id),
                product_id: Set(product_id),
                tenant_id: Set(tenant_id),
                sku: Set(var_input.sku.clone()),
                barcode: Set(var_input.barcode.clone()),
                ean: Set(None),
                upc: Set(None),
                inventory_policy: Set(var_input.inventory_policy.clone()),
                inventory_management: Set("manual".into()),
                inventory_quantity: Set(var_input.inventory_quantity),
                weight: Set(var_input.weight),
                weight_unit: Set(var_input.weight_unit.clone()),
                option1: Set(var_input.option1.clone()),
                option2: Set(var_input.option2.clone()),
                option3: Set(var_input.option3.clone()),
                position: Set(position as i32),
                created_at: Set(now.into()),
                updated_at: Set(now.into()),
            };
            variant.insert(&txn).await?;

            for price_input in &var_input.prices {
                let price = entities::price::ActiveModel {
                    id: Set(generate_id()),
                    variant_id: Set(variant_id),
                    currency_code: Set(price_input.currency_code.clone()),
                    amount: Set(price_input.amount),
                    compare_at_amount: Set(price_input.compare_at_amount),
                };
                price.insert(&txn).await?;
            }
        }

        txn.commit().await?;

        info!(product_id = %product_id, "Product created");

        let _ = self.event_bus.publish(
            tenant_id,
            Some(actor_id),
            DomainEvent::ProductCreated { product_id },
        );

        self.get_product(tenant_id, product_id).await
    }

    #[instrument(skip(self))]
    pub async fn get_product(
        &self,
        tenant_id: Uuid,
        product_id: Uuid,
    ) -> CommerceResult<ProductResponse> {
        let product = entities::product::Entity::find_by_id(product_id)
            .filter(entities::product::Column::TenantId.eq(tenant_id))
            .one(&self.db)
            .await?
            .ok_or(CommerceError::ProductNotFound(product_id))?;

        let translations = entities::product_translation::Entity::find()
            .filter(entities::product_translation::Column::ProductId.eq(product_id))
            .all(&self.db)
            .await?;

        let options = entities::product_option::Entity::find()
            .filter(entities::product_option::Column::ProductId.eq(product_id))
            .all(&self.db)
            .await?;

        let variants = entities::product_variant::Entity::find()
            .filter(entities::product_variant::Column::ProductId.eq(product_id))
            .all(&self.db)
            .await?;

        let mut variant_responses = Vec::new();
        for variant in variants {
            let prices = entities::price::Entity::find()
                .filter(entities::price::Column::VariantId.eq(variant.id))
                .all(&self.db)
                .await?;

            let price_responses = prices
                .into_iter()
                .map(|price| PriceResponse {
                    currency_code: price.currency_code,
                    amount: price.amount,
                    compare_at_amount: price.compare_at_amount,
                    on_sale: price
                        .compare_at_amount
                        .map(|c| c > price.amount)
                        .unwrap_or(false),
                })
                .collect();

            let title = Self::generate_variant_title(&variant);

            variant_responses.push(VariantResponse {
                id: variant.id,
                product_id: variant.product_id,
                sku: variant.sku,
                barcode: variant.barcode,
                title,
                option1: variant.option1,
                option2: variant.option2,
                option3: variant.option3,
                prices: price_responses,
                inventory_quantity: variant.inventory_quantity,
                inventory_policy: variant.inventory_policy.clone(),
                in_stock: variant.inventory_quantity > 0 || variant.inventory_policy == "continue",
                weight: variant.weight,
                weight_unit: variant.weight_unit,
                position: variant.position,
            });
        }

        let images = entities::product_image::Entity::find()
            .filter(entities::product_image::Column::ProductId.eq(product_id))
            .all(&self.db)
            .await?;

        Ok(ProductResponse {
            id: product.id,
            tenant_id: product.tenant_id,
            status: product.status,
            vendor: product.vendor,
            product_type: product.product_type,
            metadata: product.metadata,
            created_at: product.created_at.into(),
            updated_at: product.updated_at.into(),
            published_at: product.published_at.map(Into::into),
            translations: translations
                .into_iter()
                .map(|translation| ProductTranslationResponse {
                    locale: translation.locale,
                    title: translation.title,
                    handle: translation.handle,
                    description: translation.description,
                    meta_title: translation.meta_title,
                    meta_description: translation.meta_description,
                })
                .collect(),
            options: options
                .into_iter()
                .map(|option| ProductOptionResponse {
                    id: option.id,
                    name: option.name,
                    values: serde_json::from_value(option.values).unwrap_or_default(),
                    position: option.position,
                })
                .collect(),
            variants: variant_responses,
            images: images
                .into_iter()
                .map(|image| ProductImageResponse {
                    id: image.id,
                    media_id: image.media_id,
                    url: String::new(),
                    alt_text: image.alt_text,
                    position: image.position,
                })
                .collect(),
        })
    }

    #[instrument(skip(self, input))]
    pub async fn update_product(
        &self,
        tenant_id: Uuid,
        actor_id: Uuid,
        product_id: Uuid,
        input: UpdateProductInput,
    ) -> CommerceResult<ProductResponse> {
        let txn = self.db.begin().await?;

        let product = entities::product::Entity::find_by_id(product_id)
            .filter(entities::product::Column::TenantId.eq(tenant_id))
            .one(&txn)
            .await?
            .ok_or(CommerceError::ProductNotFound(product_id))?;

        let mut product_active: entities::product::ActiveModel = product.into();
        product_active.updated_at = Set(Utc::now().into());

        if let Some(vendor) = input.vendor {
            product_active.vendor = Set(Some(vendor));
        }
        if let Some(product_type) = input.product_type {
            product_active.product_type = Set(Some(product_type));
        }
        if let Some(metadata) = input.metadata {
            product_active.metadata = Set(metadata);
        }
        if let Some(status) = input.status {
            product_active.status = Set(status);
        }

        product_active.update(&txn).await?;

        if let Some(translations) = input.translations {
            entities::product_translation::Entity::delete_many()
                .filter(entities::product_translation::Column::ProductId.eq(product_id))
                .exec(&txn)
                .await?;

            let mut seen = HashSet::new();
            for translation_input in translations {
                let handle = translation_input
                    .handle
                    .clone()
                    .unwrap_or_else(|| Self::slugify(&translation_input.title));

                let locale = translation_input.locale.clone();
                let key = format!("{}::{}", locale, handle.clone());
                if !seen.insert(key) {
                    return Err(CommerceError::DuplicateHandle { handle, locale });
                }

                let existing = entities::product_translation::Entity::find()
                    .filter(
                        entities::product_translation::Column::Locale.eq(&translation_input.locale),
                    )
                    .filter(entities::product_translation::Column::Handle.eq(&handle))
                    .filter(entities::product_translation::Column::ProductId.ne(product_id))
                    .one(&txn)
                    .await?;
                if existing.is_some() {
                    return Err(CommerceError::DuplicateHandle {
                        handle,
                        locale: translation_input.locale,
                    });
                }

                let translation = entities::product_translation::ActiveModel {
                    id: Set(generate_id()),
                    product_id: Set(product_id),
                    locale: Set(translation_input.locale),
                    title: Set(translation_input.title),
                    handle: Set(handle),
                    description: Set(translation_input.description),
                    meta_title: Set(translation_input.meta_title),
                    meta_description: Set(translation_input.meta_description),
                };
                translation.insert(&txn).await?;
            }
        }

        txn.commit().await?;

        let _ = self.event_bus.publish(
            tenant_id,
            Some(actor_id),
            DomainEvent::ProductUpdated { product_id },
        );

        self.get_product(tenant_id, product_id).await
    }

    #[instrument(skip(self))]
    pub async fn publish_product(
        &self,
        tenant_id: Uuid,
        actor_id: Uuid,
        product_id: Uuid,
    ) -> CommerceResult<ProductResponse> {
        let product = entities::product::Entity::find_by_id(product_id)
            .filter(entities::product::Column::TenantId.eq(tenant_id))
            .one(&self.db)
            .await?
            .ok_or(CommerceError::ProductNotFound(product_id))?;

        let mut product_active: entities::product::ActiveModel = product.into();
        product_active.status = Set("active".into());
        product_active.published_at = Set(Some(Utc::now().into()));
        product_active.updated_at = Set(Utc::now().into());
        product_active.update(&self.db).await?;

        let _ = self.event_bus.publish(
            tenant_id,
            Some(actor_id),
            DomainEvent::ProductPublished { product_id },
        );

        self.get_product(tenant_id, product_id).await
    }

    #[instrument(skip(self))]
    pub async fn unpublish_product(
        &self,
        tenant_id: Uuid,
        actor_id: Uuid,
        product_id: Uuid,
    ) -> CommerceResult<ProductResponse> {
        let product = entities::product::Entity::find_by_id(product_id)
            .filter(entities::product::Column::TenantId.eq(tenant_id))
            .one(&self.db)
            .await?
            .ok_or(CommerceError::ProductNotFound(product_id))?;

        let mut product_active: entities::product::ActiveModel = product.into();
        product_active.status = Set("draft".into());
        product_active.updated_at = Set(Utc::now().into());
        product_active.update(&self.db).await?;

        let _ = self.event_bus.publish(
            tenant_id,
            Some(actor_id),
            DomainEvent::ProductUpdated { product_id },
        );

        self.get_product(tenant_id, product_id).await
    }

    #[instrument(skip(self))]
    pub async fn delete_product(
        &self,
        tenant_id: Uuid,
        actor_id: Uuid,
        product_id: Uuid,
    ) -> CommerceResult<()> {
        let product = entities::product::Entity::find_by_id(product_id)
            .filter(entities::product::Column::TenantId.eq(tenant_id))
            .one(&self.db)
            .await?
            .ok_or(CommerceError::ProductNotFound(product_id))?;

        if product.status == "active" {
            return Err(CommerceError::CannotDeletePublished);
        }

        entities::product::Entity::delete_by_id(product_id)
            .exec(&self.db)
            .await?;

        let _ = self.event_bus.publish(
            tenant_id,
            Some(actor_id),
            DomainEvent::ProductDeleted { product_id },
        );

        Ok(())
    }

    fn slugify(text: &str) -> String {
        text.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }

    fn generate_variant_title(variant: &entities::product_variant::Model) -> String {
        let options: Vec<&str> = [
            variant.option1.as_deref(),
            variant.option2.as_deref(),
            variant.option3.as_deref(),
        ]
        .into_iter()
        .flatten()
        .collect();

        if options.is_empty() {
            "Default".to_string()
        } else {
            options.join(" / ")
        }
    }
}

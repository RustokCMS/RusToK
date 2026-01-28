use sea_orm::prelude::*;
use sea_orm::EntityTrait;

use rustok_core::generate_id;

pub use super::_entities::tenants::Entity;
use super::_entities::tenants::{self, ActiveModel, Model};

impl Model {
    pub fn is_enabled(&self) -> bool {
        self.is_active
    }
}

impl ActiveModel {
    pub fn new(name: &str, slug: &str) -> Self {
        Self {
            id: sea_orm::ActiveValue::Set(generate_id()),
            name: sea_orm::ActiveValue::Set(name.to_string()),
            slug: sea_orm::ActiveValue::Set(slug.to_string()),
            domain: sea_orm::ActiveValue::NotSet,
            settings: sea_orm::ActiveValue::Set(serde_json::json!({})),
            is_active: sea_orm::ActiveValue::Set(true),
            created_at: sea_orm::ActiveValue::NotSet,
            updated_at: sea_orm::ActiveValue::NotSet,
        }
    }
}

impl Entity {
    pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Option<Model>, DbErr> {
        <Self as EntityTrait>::find_by_id(id).one(db).await
    }

    pub async fn find_by_slug(db: &DatabaseConnection, slug: &str) -> Result<Option<Model>, DbErr> {
        Self::find()
            .filter(tenants::Column::Slug.eq(slug))
            .one(db)
            .await
    }

    pub async fn find_by_domain(
        db: &DatabaseConnection,
        domain: &str,
    ) -> Result<Option<Model>, DbErr> {
        Self::find()
            .filter(tenants::Column::Domain.eq(domain))
            .one(db)
            .await
    }

    pub async fn find_active(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        Self::find()
            .filter(tenants::Column::IsActive.eq(true))
            .all(db)
            .await
    }
}

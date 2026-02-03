use async_trait::async_trait;
use rustok_core::events::{DomainEvent, EventEnvelope, EventHandler, HandlerResult};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::error::IndexResult;
use crate::traits::{Indexer, IndexerContext, LocaleIndexer};

pub struct ProductIndexer {
    db: DatabaseConnection,
}

impl ProductIndexer {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl Indexer for ProductIndexer {
    fn name(&self) -> &'static str {
        "product_indexer"
    }

    async fn index_one(&self, _ctx: &IndexerContext, _entity_id: Uuid) -> IndexResult<()> {
        Ok(())
    }

    async fn remove_one(&self, _ctx: &IndexerContext, _entity_id: Uuid) -> IndexResult<()> {
        Ok(())
    }

    async fn reindex_all(&self, _ctx: &IndexerContext) -> IndexResult<u64> {
        Ok(0)
    }
}

#[async_trait]
impl LocaleIndexer for ProductIndexer {
    async fn index_locale(
        &self,
        _ctx: &IndexerContext,
        _entity_id: Uuid,
        _locale: &str,
    ) -> IndexResult<()> {
        Ok(())
    }

    async fn remove_locale(
        &self,
        _ctx: &IndexerContext,
        _entity_id: Uuid,
        _locale: &str,
    ) -> IndexResult<()> {
        Ok(())
    }
}

#[async_trait]
impl EventHandler for ProductIndexer {
    fn name(&self) -> &'static str {
        "product_indexer"
    }

    fn handles(&self, event: &DomainEvent) -> bool {
        matches!(
            event,
            DomainEvent::ProductCreated { .. }
                | DomainEvent::ProductUpdated { .. }
                | DomainEvent::ProductPublished { .. }
                | DomainEvent::ProductDeleted { .. }
                | DomainEvent::VariantCreated { .. }
                | DomainEvent::VariantUpdated { .. }
                | DomainEvent::VariantDeleted { .. }
                | DomainEvent::InventoryUpdated { .. }
                | DomainEvent::PriceUpdated { .. }
        ) || matches!(
            event,
            DomainEvent::ReindexRequested { target_type, .. } if target_type == "product"
        )
    }

    async fn handle(&self, envelope: &EventEnvelope) -> HandlerResult {
        let ctx = IndexerContext::new(self.db.clone(), envelope.tenant_id);

        match &envelope.event {
            DomainEvent::ProductCreated { product_id }
            | DomainEvent::ProductUpdated { product_id }
            | DomainEvent::ProductPublished { product_id } => {
                self.index_one(&ctx, *product_id).await?;
            }

            DomainEvent::ProductDeleted { product_id } => {
                self.remove_one(&ctx, *product_id).await?;
            }

            DomainEvent::VariantCreated { product_id, .. }
            | DomainEvent::VariantUpdated { product_id, .. }
            | DomainEvent::VariantDeleted { product_id, .. } => {
                self.index_one(&ctx, *product_id).await?;
            }

            DomainEvent::InventoryUpdated { product_id, .. } => {
                self.index_one(&ctx, *product_id).await?;
            }

            DomainEvent::PriceUpdated { product_id, .. } => {
                self.index_one(&ctx, *product_id).await?;
            }

            DomainEvent::ReindexRequested { target_id, .. } => {
                if let Some(id) = target_id {
                    self.index_one(&ctx, *id).await?;
                } else {
                    self.reindex_all(&ctx).await?;
                }
            }

            _ => {}
        }

        Ok(())
    }
}

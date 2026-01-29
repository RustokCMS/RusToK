use async_trait::async_trait;
use rustok_core::{DomainEvent, EventEnvelope, EventHandler, Result};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::stats::IndexStats;

pub struct ProductIndexer {
    db: DatabaseConnection,
}

impl ProductIndexer {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn index_product(&self, _product_id: Uuid) -> Result<()> {
        Ok(())
    }

    pub async fn reindex_tenant(&self, _tenant_id: Uuid) -> Result<IndexStats> {
        let mut stats = IndexStats::default();
        stats.record_success();
        Ok(stats)
    }
}

#[async_trait]
impl EventHandler for ProductIndexer {
    fn handles(&self, event: &DomainEvent) -> bool {
        matches!(
            event,
            DomainEvent::ProductCreated { .. }
                | DomainEvent::ProductUpdated { .. }
                | DomainEvent::ProductPublished { .. }
                | DomainEvent::VariantUpdated { .. }
                | DomainEvent::InventoryUpdated { .. }
        )
    }

    async fn handle(&self, envelope: &EventEnvelope) -> Result<()> {
        let product_id = match &envelope.event {
            DomainEvent::ProductCreated { product_id }
            | DomainEvent::ProductUpdated { product_id }
            | DomainEvent::ProductPublished { product_id }
            | DomainEvent::VariantUpdated { product_id, .. } => *product_id,
            DomainEvent::InventoryUpdated { .. } => return Ok(()),
            _ => return Ok(()),
        };

        self.index_product(product_id).await
    }
}

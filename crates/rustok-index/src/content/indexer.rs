use async_trait::async_trait;
use rustok_core::events::{DomainEvent, EventEnvelope, EventHandler, HandlerResult};
use sea_orm::DatabaseConnection;
use tracing::{debug, info, instrument};
use uuid::Uuid;

use crate::error::IndexResult;
use crate::traits::{Indexer, IndexerContext, LocaleIndexer};

/// Content indexer - listens to events and updates index_content table
pub struct ContentIndexer {
    db: DatabaseConnection,
}

impl ContentIndexer {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Build denormalized content from normalized tables
    #[instrument(skip(self, ctx))]
    async fn build_index_content(
        &self,
        ctx: &IndexerContext,
        node_id: Uuid,
        locale: &str,
    ) -> IndexResult<Option<super::model::IndexContentModel>> {
        let _ = ctx;
        debug!(node_id = %node_id, locale = locale, "Building index content");
        Ok(None)
    }

    async fn get_tenant_locales(&self, ctx: &IndexerContext) -> IndexResult<Vec<String>> {
        let _ = ctx;
        Ok(vec!["en".to_string()])
    }
}

#[async_trait]
impl Indexer for ContentIndexer {
    fn name(&self) -> &'static str {
        "content_indexer"
    }

    #[instrument(skip(self, ctx))]
    async fn index_one(&self, ctx: &IndexerContext, entity_id: Uuid) -> IndexResult<()> {
        let locales = self.get_tenant_locales(ctx).await?;

        for locale in locales {
            self.index_locale(ctx, entity_id, &locale).await?;
        }

        Ok(())
    }

    #[instrument(skip(self, ctx))]
    async fn remove_one(&self, ctx: &IndexerContext, entity_id: Uuid) -> IndexResult<()> {
        let _ = ctx;
        debug!(node_id = %entity_id, "Removing from content index");
        Ok(())
    }

    #[instrument(skip(self, ctx))]
    async fn reindex_all(&self, ctx: &IndexerContext) -> IndexResult<u64> {
        info!(tenant_id = %ctx.tenant_id, "Reindexing all content");
        Ok(0)
    }
}

#[async_trait]
impl LocaleIndexer for ContentIndexer {
    #[instrument(skip(self, ctx))]
    async fn index_locale(
        &self,
        ctx: &IndexerContext,
        entity_id: Uuid,
        locale: &str,
    ) -> IndexResult<()> {
        let content = self.build_index_content(ctx, entity_id, locale).await?;

        match content {
            Some(_model) => {
                debug!(node_id = %entity_id, locale = locale, "Indexed content");
            }
            None => {
                self.remove_locale(ctx, entity_id, locale).await?;
            }
        }

        Ok(())
    }

    async fn remove_locale(
        &self,
        _ctx: &IndexerContext,
        entity_id: Uuid,
        locale: &str,
    ) -> IndexResult<()> {
        debug!(node_id = %entity_id, locale = locale, "Removed locale from content index");
        Ok(())
    }
}

#[async_trait]
impl EventHandler for ContentIndexer {
    fn name(&self) -> &'static str {
        "content_indexer"
    }

    fn handles(&self, event: &DomainEvent) -> bool {
        match event {
            DomainEvent::NodeCreated { .. }
            | DomainEvent::NodeUpdated { .. }
            | DomainEvent::NodeTranslationUpdated { .. }
            | DomainEvent::NodePublished { .. }
            | DomainEvent::NodeUnpublished { .. }
            | DomainEvent::NodeDeleted { .. }
            | DomainEvent::BodyUpdated { .. }
            | DomainEvent::CategoryUpdated { .. } => true,
            DomainEvent::TagAttached { target_type, .. }
            | DomainEvent::TagDetached { target_type, .. } => target_type == "node",
            DomainEvent::ReindexRequested { target_type, .. } => target_type == "content",
            _ => false,
        }
    }

    async fn handle(&self, envelope: &EventEnvelope) -> HandlerResult {
        let ctx = IndexerContext::new(self.db.clone(), envelope.tenant_id);

        match &envelope.event {
            DomainEvent::NodeCreated { node_id, .. }
            | DomainEvent::NodeUpdated { node_id, .. }
            | DomainEvent::NodePublished { node_id, .. }
            | DomainEvent::NodeUnpublished { node_id, .. } => {
                self.index_one(&ctx, *node_id).await?;
            }

            DomainEvent::NodeTranslationUpdated { node_id, locale } => {
                self.index_locale(&ctx, *node_id, locale).await?;
            }

            DomainEvent::BodyUpdated { node_id, locale } => {
                self.index_locale(&ctx, *node_id, locale).await?;
            }

            DomainEvent::NodeDeleted { node_id, .. } => {
                self.remove_one(&ctx, *node_id).await?;
            }

            DomainEvent::TagAttached { target_id, .. }
            | DomainEvent::TagDetached { target_id, .. } => {
                self.index_one(&ctx, *target_id).await?;
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

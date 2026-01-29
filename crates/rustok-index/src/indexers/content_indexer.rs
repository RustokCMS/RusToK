use async_trait::async_trait;
use rustok_core::{DomainEvent, EventEnvelope, EventHandler, Result};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct ContentIndexer {
    db: DatabaseConnection,
}

impl ContentIndexer {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn index_node(&self, _node_id: Uuid) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
impl EventHandler for ContentIndexer {
    fn handles(&self, event: &DomainEvent) -> bool {
        matches!(
            event,
            DomainEvent::NodeCreated { .. }
                | DomainEvent::NodeUpdated { .. }
                | DomainEvent::NodePublished { .. }
        )
    }

    async fn handle(&self, envelope: &EventEnvelope) -> Result<()> {
        let node_id = match &envelope.event {
            DomainEvent::NodeCreated { node_id, .. }
            | DomainEvent::NodeUpdated { node_id }
            | DomainEvent::NodePublished { node_id, .. } => *node_id,
            _ => return Ok(()),
        };

        self.index_node(node_id).await
    }
}

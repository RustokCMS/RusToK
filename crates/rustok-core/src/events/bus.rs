use chrono::Utc;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::id::generate_id;

use super::types::{DomainEvent, EventEnvelope};

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<EventEnvelope>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn publish(&self, tenant_id: Uuid, event: DomainEvent) {
        let envelope = EventEnvelope {
            id: generate_id(),
            tenant_id,
            timestamp: Utc::now(),
            payload: event,
        };

        if self.sender.send(envelope.clone()).is_err() {
            tracing::debug!(event_id = ?envelope.id, "Event publish failed");
        } else {
            tracing::debug!(
                event_id = ?envelope.id,
                event_type = ?envelope.payload,
                "Event published"
            );
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<EventEnvelope> {
        self.sender.subscribe()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(1024)
    }
}

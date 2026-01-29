use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct EventEnvelope {
    pub id: Uuid,
    pub occurred_at: DateTime<Utc>,
    pub event: DomainEvent,
}

#[derive(Debug, Clone)]
pub enum DomainEvent {
    ModuleEnabled {
        tenant_id: Uuid,
        module_slug: String,
    },
    ModuleDisabled {
        tenant_id: Uuid,
        module_slug: String,
    },
}

pub trait EventHandler: Send + Sync {
    fn handles(&self, event: &DomainEvent) -> bool;
    fn name(&self) -> &'static str;
    fn handle(&self, envelope: &EventEnvelope) -> crate::Result<()>;
}

#[derive(Clone, Default)]
pub struct EventBus;

impl EventBus {
    pub fn new() -> Self {
        Self
    }

    pub fn subscribe(&self) {
        tracing::debug!("EventBus subscribe (stub)");
    }

    pub fn publish(&self, event: DomainEvent) -> crate::Result<()> {
        tracing::debug!(?event, "Event published (stub)");
        Ok(())
    }
}

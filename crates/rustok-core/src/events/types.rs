use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub payload: DomainEvent,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum DomainEvent {
    // ============ Core Content Events ============
    NodeCreated { node_id: Uuid, kind: String },
    NodeUpdated { node_id: Uuid },
    NodePublished { node_id: Uuid, kind: String },
    NodeDeleted { node_id: Uuid },

    NodeTranslationUpdated { node_id: Uuid, locale: String },

    // ============ Commerce Events (Placeholder) ============
    ProductCreated { product_id: Uuid },
    ProductUpdated { product_id: Uuid },
    OrderPlaced { order_id: Uuid, total: i64 },

    // ============ User Events ============
    UserRegistered { user_id: Uuid, email: String },
    UserLoggedIn { user_id: Uuid },

    // ============ System Events ============
    ModuleEnabled { module_slug: String },
    CacheCleared { target: String },
}

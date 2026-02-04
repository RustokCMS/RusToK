use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TriggerEvent {
    BeforeCreate,
    BeforeUpdate,
    BeforeDelete,
    AfterCreate,
    AfterUpdate,
    AfterDelete,
    OnCommit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptTrigger {
    Event { entity: String, event: TriggerEvent },
    Cron(String),
    Manual,
    ApiEndpoint { method: String, path: String },
}

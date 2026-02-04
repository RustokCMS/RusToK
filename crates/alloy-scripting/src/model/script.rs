use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::trigger::ScriptTrigger;

pub type ScriptId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptStatus {
    Active,
    Disabled,
    Draft,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub id: ScriptId,
    pub name: String,
    pub description: Option<String>,
    pub code: String,
    pub trigger: ScriptTrigger,
    pub status: ScriptStatus,
    pub version: u32,
    pub run_as_system: bool,
    pub permissions: Vec<String>,
    pub author_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub error_count: u32,
    pub last_error_at: Option<DateTime<Utc>>,
}

impl Script {
    pub fn is_executable(&self) -> bool {
        matches!(self.status, ScriptStatus::Active)
    }
}

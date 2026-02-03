use serde::{Deserialize, Serialize};
use uuid::Uuid;

const DEFAULT_TENANT_ID: Uuid = Uuid::from_u128(1);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RustokSettings {
    #[serde(default)]
    pub tenant: TenantSettings,
    #[serde(default)]
    pub search: SearchSettings,
    #[serde(default)]
    pub features: FeatureSettings,
    #[serde(default)]
    pub rate_limit: RateLimitSettings,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TenantSettings {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_resolution")]
    pub resolution: String,
    #[serde(default = "default_header_name")]
    pub header_name: String,
    #[serde(default = "default_tenant_id")]
    pub default_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeatureSettings {
    #[serde(default = "default_true")]
    pub registration_enabled: bool,
    #[serde(default)]
    pub email_verification: bool,
    #[serde(default = "default_true")]
    pub multi_tenant: bool,
    #[serde(default = "default_true")]
    pub search_indexing: bool,
    #[serde(default)]
    pub oauth_enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchSettings {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_search_driver")]
    pub driver: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default = "default_index_prefix")]
    pub index_prefix: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RateLimitSettings {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_requests_per_minute")]
    pub requests_per_minute: u32,
    #[serde(default = "default_burst")]
    pub burst: u32,
}

impl Default for TenantSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            resolution: default_resolution(),
            header_name: default_header_name(),
            default_id: default_tenant_id(),
        }
    }
}

impl Default for FeatureSettings {
    fn default() -> Self {
        Self {
            registration_enabled: true,
            email_verification: false,
            multi_tenant: true,
            search_indexing: true,
            oauth_enabled: false,
        }
    }
}

impl Default for SearchSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            driver: default_search_driver(),
            url: String::new(),
            api_key: None,
            index_prefix: default_index_prefix(),
        }
    }
}

impl Default for RateLimitSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            requests_per_minute: default_requests_per_minute(),
            burst: default_burst(),
        }
    }
}

impl RustokSettings {
    pub fn from_settings(settings: &Option<serde_json::Value>) -> Result<Self, serde_json::Error> {
        let root = settings
            .clone()
            .unwrap_or_else(|| serde_json::json!({}));
        let rustok = root.get("rustok").cloned().unwrap_or_else(|| serde_json::json!({}));
        serde_json::from_value(rustok)
    }
}

fn default_tenant_id() -> Uuid {
    DEFAULT_TENANT_ID
}

fn default_resolution() -> String {
    "header".to_string()
}

fn default_header_name() -> String {
    "X-Tenant-ID".to_string()
}

fn default_true() -> bool {
    true
}

fn default_search_driver() -> String {
    "meilisearch".to_string()
}

fn default_index_prefix() -> String {
    "rustok_".to_string()
}

fn default_requests_per_minute() -> u32 {
    60
}

fn default_burst() -> u32 {
    10
}

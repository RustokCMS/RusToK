use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Information about a RusToK module
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModuleInfo {
    /// Unique slug identifier for the module
    pub slug: String,
    /// Human-readable name of the module
    pub name: String,
    /// Description of the module's functionality
    pub description: String,
    /// Version of the module
    pub version: String,
    /// List of module dependencies
    pub dependencies: Vec<String>,
}

/// Response containing a list of modules
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModuleListResponse {
    /// List of available modules
    pub modules: Vec<ModuleInfo>,
}

/// Request to check if a module exists
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModuleLookupRequest {
    /// The slug of the module to look up
    pub slug: String,
}

/// Response indicating whether a module exists
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModuleLookupResponse {
    /// The slug that was queried
    pub slug: String,
    /// Whether the module exists
    pub exists: bool,
}

use std::sync::Arc;

use anyhow::Result;
use rmcp::{
    model::{CallToolRequestParams, CallToolResult, Implementation, ListToolsResult, ServerInfo},
    service::{RequestContext, RoleServer},
    transport::io::stdio,
    ServerHandler,
};
use rustok_core::registry::ModuleRegistry;

use crate::tools::{ModuleLookupRequest, ModuleLookupResponse, ModuleListResponse};

/// Configuration for the MCP server
pub struct McpServerConfig {
    pub registry: ModuleRegistry,
}

impl McpServerConfig {
    pub fn new(registry: ModuleRegistry) -> Self {
        Self { registry }
    }
}

/// MCP Server handler for RusToK modules
#[derive(Clone)]
pub struct RusToKMcpServer {
    registry: Arc<ModuleRegistry>,
}

impl RusToKMcpServer {
    pub fn new(registry: ModuleRegistry) -> Self {
        Self {
            registry: Arc::new(registry),
        }
    }

    /// List all registered modules
    async fn list_modules_internal(&self) -> ModuleListResponse {
        let modules = self
            .registry
            .list()
            .into_iter()
            .map(|module| crate::tools::ModuleInfo {
                slug: module.slug().to_string(),
                name: module.name().to_string(),
                description: module.description().to_string(),
                version: module.version().to_string(),
                dependencies: module
                    .dependencies()
                    .iter()
                    .map(|dep| dep.to_string())
                    .collect(),
            })
            .collect();

        ModuleListResponse { modules }
    }

    /// Check if a module exists by slug
    async fn module_exists_internal(&self, slug: &str) -> ModuleLookupResponse {
        let exists = self.registry.contains(slug);

        ModuleLookupResponse {
            slug: slug.to_string(),
            exists,
        }
    }
}

// Manual implementation of tool handling without macros for better control
impl ServerHandler for RusToKMcpServer {
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        match request.name.as_ref() {
            "list_modules" => {
                let result = self.list_modules_internal().await;
                let content = serde_json::to_string(&result).map_err(|e| {
                    rmcp::ErrorData::internal_error(
                        format!("Failed to serialize response: {}", e),
                        None,
                    )
                })?;
                Ok(CallToolResult::success(vec![rmcp::model::Content::text(
                    content,
                )]))
            }
            "module_exists" => {
                let args = request.arguments.ok_or_else(|| {
                    rmcp::ErrorData::invalid_params("Missing arguments", None)
                })?;
                let req: ModuleLookupRequest = serde_json::from_value(
                    serde_json::Value::Object(args),
                )
                .map_err(|e| {
                    rmcp::ErrorData::invalid_params(
                        format!("Invalid arguments: {}", e),
                        None,
                    )
                })?;
                let result = self.module_exists_internal(&req.slug).await;
                let content = serde_json::to_string(&result).map_err(|e| {
                    rmcp::ErrorData::internal_error(
                        format!("Failed to serialize response: {}", e),
                        None,
                    )
                })?;
                Ok(CallToolResult::success(vec![rmcp::model::Content::text(
                    content,
                )]))
            }
            _ => Err(rmcp::ErrorData::new(
                rmcp::model::ErrorCode::METHOD_NOT_FOUND,
                format!("Unknown tool: {}", request.name),
                None,
            )),
        }
    }

    async fn list_tools(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, rmcp::ErrorData> {
        use rmcp::model::Tool;
        use schemars::schema_for;

        // Convert schema to JsonObject (serde_json::Map<String, Value>)
        let list_modules_schema = match serde_json::to_value(schema_for!(crate::tools::ModuleListResponse)) {
            Ok(serde_json::Value::Object(map)) => map,
            _ => serde_json::Map::new(),
        };

        let module_exists_schema = match serde_json::to_value(schema_for!(crate::tools::ModuleLookupRequest)) {
            Ok(serde_json::Value::Object(map)) => map,
            _ => serde_json::Map::new(),
        };

        let tools = vec![
            Tool::new(
                "list_modules",
                "List all registered RusToK modules with their metadata",
                list_modules_schema,
            ),
            Tool::new(
                "module_exists",
                "Check if a module exists by its slug",
                module_exists_schema,
            ),
        ];

        Ok(ListToolsResult {
            tools,
            next_cursor: None,
            meta: None,
        })
    }

    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: rmcp::model::ProtocolVersion::V_2024_11_05,
            capabilities: rmcp::model::ServerCapabilities::default(),
            server_info: Implementation {
                name: "RusToK MCP Server".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                title: Some("RusToK MCP Server".to_string()),
                description: Some(
                    "MCP server for exploring RusToK modules. Use list_modules to see all available modules and module_exists to check specific modules.".to_string(),
                ),
                icons: None,
                website_url: None,
            },
            instructions: Some(
                "MCP server for exploring RusToK modules. Use list_modules to see all available modules and module_exists to check specific modules.".to_string(),
            ),
        }
    }
}

/// Serve the MCP server over stdio
pub async fn serve_stdio(config: McpServerConfig) -> Result<()> {
    let server = RusToKMcpServer::new(config.registry);
    let transport = stdio();

    let service = rmcp::serve_server(server, transport).await?;

    // Wait for the service to complete (e.g., when stdin closes)
    service.waiting().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_creation() {
        let registry = ModuleRegistry::new();
        let server = RusToKMcpServer::new(registry);

        let info = server.get_info();
        assert_eq!(info.server_info.name, "RusToK MCP Server");
        assert!(info.instructions.is_some());
    }
}

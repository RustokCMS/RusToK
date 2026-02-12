//! # Test Server Utilities
//!
//! Provides utilities for spawning a test server for integration testing.

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::net::TcpListener;
use axum::Router;
use loco_rs::app::AppContext;
use loco_rs::boot::create_app;
use loco_rs::config::Config;
use loco_rs::environment::Environment;
use migration::Migrator;
use rustok_server::app::App;

/// Test server handle that can be used to manage the server lifecycle
pub struct TestServer {
    /// Base URL of the test server
    pub base_url: String,
    /// Shutdown sender
    shutdown_tx: Option<oneshot::Sender<()>>,
    /// App context for direct access to services
    pub ctx: Arc<AppContext>,
}

impl TestServer {
    /// Spawn a new test server
    ///
    /// This will:
    /// 1. Find an available port
    /// 2. Create a test database connection
    /// 3. Run migrations
    /// 4. Start the server
    /// 5. Return a handle with the base URL
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rustok_test_utils::test_server::TestServer;
    ///
    /// #[tokio::test]
    /// async fn test_with_server() {
    ///     let server = TestServer::spawn().await.unwrap();
    ///
    ///     // Make HTTP requests to server.base_url
    ///     let client = reqwest::Client::new();
    ///     let response = client.get(&format!("{}/health", server.base_url))
    ///         .send()
    ///         .await
    ///         .unwrap();
    ///
    ///     // Server is automatically shut down when `server` is dropped
    /// }
    /// ```
    pub async fn spawn() -> Result<Self, TestServerError> {
        Self::spawn_with_config(None).await
    }

    /// Spawn a test server with custom database URL
    ///
    /// # Arguments
    ///
    /// * `database_url` - Optional custom database URL. If None, uses in-memory SQLite.
    pub async fn spawn_with_config(
        database_url: Option<String>,
    ) -> Result<Self, TestServerError> {
        // Find an available port
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .map_err(|e| TestServerError::BindError(e.to_string()))?;
        let port = listener
            .local_addr()
            .map_err(|e| TestServerError::BindError(e.to_string()))?
            .port();

        let base_url = format!("http://127.0.0.1:{}", port);

        // Create test database
        let db_url = database_url.unwrap_or_else(|| "sqlite::memory:".to_string());
        let config = Self::create_test_config(&db_url)?;

        // Boot the application
        let boot_result = create_app::<App, Migrator>(
            loco_rs::boot::StartMode::Server,
            &Environment::Test,
            config,
        )
        .await
        .map_err(|e| TestServerError::BootError(e.to_string()))?;

        let ctx = Arc::new(boot_result.ctx);

        // Run migrations
        Migrator::up(&ctx.db, None)
            .await
            .map_err(|e| TestServerError::MigrationError(e.to_string()))?;

        // Create router
        let router = boot_result.router;

        // Setup shutdown channel
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();

        // Spawn server task
        let server_handle = tokio::spawn(async move {
            axum::serve(listener, router)
                .with_graceful_shutdown(async move {
                    // Wait for shutdown signal
                    let _ = shutdown_rx.await;
                })
                .await
        });

        // Store the server handle for cleanup
        tokio::spawn(async move {
            // Abort server on drop
            if let Ok(()) = server_handle.await {
                // Server completed normally
            }
        });

        Ok(Self {
            base_url,
            shutdown_tx: Some(shutdown_tx),
            ctx,
        })
    }

    /// Create test configuration
    fn create_test_config(database_url: &str) -> Result<Config, TestServerError> {
        // Parse basic configuration
        let mut config = Config::new().map_err(|e| TestServerError::ConfigError(e.to_string()))?;

        // Override database URL
        config.database.dialect = "sqlite".to_string();
        config.database.uri = database_url.to_string();
        config.database.max_connections = 5;
        config.database.min_connections = 1;
        config.database.connect_timeout = 5;

        // Set test-specific settings
        config.environment = "test".to_string();
        config.server.worker_processes = 1;

        Ok(config)
    }

    /// Get the database connection from the app context
    pub fn db(&self) -> &sea_orm::DatabaseConnection {
        &self.ctx.db
    }

    /// Shutdown the test server
    pub async fn shutdown(mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }

        // Wait a bit for graceful shutdown
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Shutdown app context
        App::shutdown(&self.ctx).await;
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            // Send shutdown signal (non-blocking in drop)
            let _ = tx.send(());
        }
    }
}

/// Errors that can occur when spawning a test server
#[derive(Debug, thiserror::Error)]
pub enum TestServerError {
    #[error("Bind error: {0}")]
    BindError(String),

    #[error("Boot error: {0}")]
    BootError(String),

    #[error("Migration error: {0}")]
    MigrationError(String),

    #[error("Config error: {0}")]
    ConfigError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_spawn() {
        let server = TestServer::spawn().await.unwrap();

        // Verify we got a URL
        assert!(server.base_url.starts_with("http://127.0.0.1:"));

        // Verify we can access the database
        let db = server.db();
        assert_eq!(db.ping().await, 0);
    }

    #[tokio::test]
    async fn test_server_health_check() {
        let server = TestServer::spawn().await.unwrap();
        let client = reqwest::Client::new();

        let response = client
            .get(format!("{}/health", server.base_url))
            .send()
            .await
            .unwrap();

        assert!(response.status().is_success());
    }
}

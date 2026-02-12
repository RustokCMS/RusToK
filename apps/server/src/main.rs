use loco_rs::cli;
use migration::Migrator;
use rustok_server::app::App;
use rustok_telemetry::{LogFormat, TelemetryConfig};
use rustok_telemetry::otel::{OtelConfig, init_tracing, shutdown};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Initialize OpenTelemetry tracing first
    let otel_config = OtelConfig::from_env();
    if let Err(e) = init_tracing(otel_config).await {
        eprintln!("Warning: Failed to initialize OpenTelemetry: {}", e);
    }

    // Initialize basic telemetry (logging, metrics)
    let _telemetry = rustok_telemetry::init(telemetry_config())?;

    // Run the application
    let result = cli::main::<App, Migrator>().await;

    // Graceful shutdown of OpenTelemetry
    shutdown().await;

    Ok(result?)
}

fn telemetry_config() -> TelemetryConfig {
    let log_format = match std::env::var("RUSTOK_LOG_FORMAT").as_deref() {
        Ok("json") => LogFormat::Json,
        _ => LogFormat::Pretty,
    };
    let metrics = std::env::var("RUSTOK_METRICS")
        .map(|value| value != "0")
        .unwrap_or(true);

    TelemetryConfig {
        service_name: "rustok-server".to_string(),
        log_format,
        metrics,
    }
}

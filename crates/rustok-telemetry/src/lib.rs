pub mod otel;
pub mod metrics;

use once_cell::sync::OnceCell;
use prometheus::{IntGauge, Encoder, TextEncoder, Registry};
use std::sync::Arc;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Layer, Registry as TracingRegistry};
use lazy_static::lazy_static;

static METRICS_HANDLE: OnceCell<Arc<MetricsHandle>> = OnceCell::new();
static REGISTRY: OnceCell<Registry> = OnceCell::new();

#[derive(Clone, Debug)]
pub struct MetricsHandle {
    registry: Arc<Registry>,
}

impl MetricsHandle {
    pub fn new() -> Self {
        let registry = Registry::new();
        Self {
            registry: Arc::new(registry),
        }
    }

    pub fn render(&self) -> String {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).ok();
        String::from_utf8(buffer).unwrap_or_else(|_| String::from("Failed to encode metrics"))
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LogFormat {
    Json,
    Pretty,
}

#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    pub service_name: String,
    pub log_format: LogFormat,
    pub metrics: bool,
}

#[derive(Clone)]
pub struct TelemetryHandles {
    pub metrics: Option<Arc<MetricsHandle>>,
}

impl std::fmt::Debug for TelemetryHandles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TelemetryHandles")
            .field("metrics", &self.metrics.is_some())
            .finish()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TelemetryError {
    #[error("failed to set global tracing subscriber")]
    SubscriberAlreadySet,
    #[error("prometheus registry error: {0}")]
    Prometheus(#[from] prometheus::Error),
}

use prometheus::{CounterVec, HistogramVec, Opts, HistogramOpts, IntCounter, IntCounterVec, IntGaugeVec};

lazy_static! {
    // Content metrics
    pub static ref CONTENT_OPERATIONS_TOTAL: CounterVec = CounterVec::new(
        Opts::new("rustok_content_operations_total", "Total content operations"),
        &["operation", "kind", "status"]
    ).expect("Failed to create content_operations_total");

    pub static ref CONTENT_OPERATION_DURATION_SECONDS: HistogramVec = HistogramVec::new(
        HistogramOpts::new("rustok_content_operation_duration_seconds", "Duration of content operations"),
        &["operation", "kind"]
    ).expect("Failed to create content_operation_duration_seconds");

    pub static ref CONTENT_NODES_TOTAL: IntGauge = IntGauge::new(
        "rustok_content_nodes_total",
        "Total number of content nodes"
    ).expect("Failed to create content_nodes_total");

    // Commerce metrics
    pub static ref COMMERCE_OPERATIONS_TOTAL: CounterVec = CounterVec::new(
        Opts::new("rustok_commerce_operations_total", "Total commerce operations"),
        &["operation", "kind", "status"]
    ).expect("Failed to create commerce_operations_total");

    pub static ref COMMERCE_OPERATION_DURATION_SECONDS: HistogramVec = HistogramVec::new(
        HistogramOpts::new("rustok_commerce_operation_duration_seconds", "Duration of commerce operations"),
        &["operation", "kind"]
    ).expect("Failed to create commerce_operation_duration_seconds");

    pub static ref COMMERCE_PRODUCTS_TOTAL: IntGauge = IntGauge::new(
        "rustok_commerce_products_total",
        "Total number of products"
    ).expect("Failed to create commerce_products_total");

    pub static ref COMMERCE_ORDERS_TOTAL: IntGauge = IntGauge::new(
        "rustok_commerce_orders_total",
        "Total number of orders"
    ).expect("Failed to create commerce_orders_total");

    // HTTP metrics
    pub static ref HTTP_REQUESTS_TOTAL: CounterVec = CounterVec::new(
        Opts::new("rustok_http_requests_total", "Total HTTP requests"),
        &["method", "path", "status"]
    ).expect("Failed to create http_requests_total");

    pub static ref HTTP_REQUEST_DURATION_SECONDS: HistogramVec = HistogramVec::new(
        HistogramOpts::new("rustok_http_request_duration_seconds", "HTTP request duration"),
        &["method", "path"]
    ).expect("Failed to create http_request_duration_seconds");

    // EventBus metrics
    pub static ref EVENTBUS_EVENTS_PUBLISHED_TOTAL: IntCounter = IntCounter::new(
        "rustok_eventbus_events_published_total",
        "Total number of events published to the EventBus"
    ).expect("Failed to create eventbus_events_published_total");

    pub static ref EVENTBUS_EVENTS_DROPPED_TOTAL: IntCounter = IntCounter::new(
        "rustok_eventbus_events_dropped_total",
        "Total number of events dropped from the EventBus"
    ).expect("Failed to create eventbus_events_dropped_total");

    pub static ref EVENTBUS_SUBSCRIBERS: IntGauge = IntGauge::new(
        "rustok_eventbus_subscribers",
        "Current number of EventBus subscribers"
    ).expect("Failed to create eventbus_subscribers");

    pub static ref EVENTBUS_EVENTS_PROCESSED_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("rustok_eventbus_events_processed_total", "Total number of events processed by subscribers"),
        &["module", "status"]
    ).expect("Failed to create eventbus_events_processed_total");

    pub static ref EVENTBUS_PROCESSING_DURATION_SECONDS: HistogramVec = HistogramVec::new(
        HistogramOpts::new("rustok_eventbus_processing_duration_seconds", "Duration of event processing by subscribers"),
        &["module"]
    ).expect("Failed to create eventbus_processing_duration_seconds");

    // Circuit Breaker metrics
    pub static ref CIRCUIT_BREAKER_STATE: IntGaugeVec = IntGaugeVec::new(
        Opts::new("rustok_circuit_breaker_state", "Current circuit breaker state (0=Closed, 1=Open, 2=HalfOpen)"),
        &["name", "service"]
    ).expect("Failed to create circuit_breaker_state");

    pub static ref CIRCUIT_BREAKER_REQUESTS_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("rustok_circuit_breaker_requests_total", "Total number of requests through circuit breaker"),
        &["name", "service"]
    ).expect("Failed to create circuit_breaker_requests_total");

    pub static ref CIRCUIT_BREAKER_SUCCESSES_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("rustok_circuit_breaker_successes_total", "Total number of successful requests through circuit breaker"),
        &["name", "service"]
    ).expect("Failed to create circuit_breaker_successes_total");

    pub static ref CIRCUIT_BREAKER_FAILURES_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("rustok_circuit_breaker_failures_total", "Total number of failed requests through circuit breaker"),
        &["name", "service"]
    ).expect("Failed to create circuit_breaker_failures_total");

    pub static ref CIRCUIT_BREAKER_REJECTED_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("rustok_circuit_breaker_rejected_total", "Total number of requests rejected by circuit breaker"),
        &["name", "service"]
    ).expect("Failed to create circuit_breaker_rejected_total");

    pub static ref CIRCUIT_BREAKER_STATE_TRANSITIONS_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("rustok_circuit_breaker_state_transitions_total", "Total number of circuit breaker state transitions"),
        &["name", "service", "from_state", "to_state"]
    ).expect("Failed to create circuit_breaker_state_transitions_total");

    // Error rate metrics
    pub static ref ERROR_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("rustok_errors_total", "Total number of errors"),
        &["module", "error_type", "severity"]
    ).expect("Failed to create error_total");

    // Tenant cache metrics (already exposed via custom metrics in controller)
    pub static ref TENANT_CACHE_HITS: IntCounter = IntCounter::new(
        "rustok_tenant_cache_hits",
        "Total number of tenant cache hits"
    ).expect("Failed to create tenant_cache_hits");

    pub static ref TENANT_CACHE_MISSES: IntCounter = IntCounter::new(
        "rustok_tenant_cache_misses",
        "Total number of tenant cache misses"
    ).expect("Failed to create tenant_cache_misses");
}

pub fn init(config: TelemetryConfig) -> Result<TelemetryHandles, TelemetryError> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let fmt_layer: Box<dyn Layer<_> + Send + Sync> = match config.log_format {
        LogFormat::Json => fmt::layer()
            .with_span_events(fmt::format::FmtSpan::CLOSE)
            .json()
            .boxed(),
        LogFormat::Pretty => fmt::layer()
            .with_span_events(fmt::format::FmtSpan::CLOSE)
            .pretty()
            .boxed(),
    };

    let subscriber = TracingRegistry::default().with(env_filter).with(fmt_layer);
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|_| TelemetryError::SubscriberAlreadySet)?;

    let metrics_handle = if config.metrics {
        let handle = Arc::new(MetricsHandle::new());
        let registry = handle.registry();

        // Register all metrics
        // Content metrics
        registry.register(Box::new(CONTENT_OPERATIONS_TOTAL.clone()))?;
        registry.register(Box::new(CONTENT_OPERATION_DURATION_SECONDS.clone()))?;
        registry.register(Box::new(CONTENT_NODES_TOTAL.clone()))?;

        // Commerce metrics
        registry.register(Box::new(COMMERCE_OPERATIONS_TOTAL.clone()))?;
        registry.register(Box::new(COMMERCE_OPERATION_DURATION_SECONDS.clone()))?;
        registry.register(Box::new(COMMERCE_PRODUCTS_TOTAL.clone()))?;
        registry.register(Box::new(COMMERCE_ORDERS_TOTAL.clone()))?;

        // HTTP metrics
        registry.register(Box::new(HTTP_REQUESTS_TOTAL.clone()))?;
        registry.register(Box::new(HTTP_REQUEST_DURATION_SECONDS.clone()))?;

        // EventBus metrics
        registry.register(Box::new(EVENTBUS_EVENTS_PUBLISHED_TOTAL.clone()))?;
        registry.register(Box::new(EVENTBUS_EVENTS_DROPPED_TOTAL.clone()))?;
        registry.register(Box::new(EVENTBUS_SUBSCRIBERS.clone()))?;
        registry.register(Box::new(EVENTBUS_EVENTS_PROCESSED_TOTAL.clone()))?;
        registry.register(Box::new(EVENTBUS_PROCESSING_DURATION_SECONDS.clone()))?;

        // Circuit Breaker metrics
        registry.register(Box::new(CIRCUIT_BREAKER_STATE.clone()))?;
        registry.register(Box::new(CIRCUIT_BREAKER_REQUESTS_TOTAL.clone()))?;
        registry.register(Box::new(CIRCUIT_BREAKER_SUCCESSES_TOTAL.clone()))?;
        registry.register(Box::new(CIRCUIT_BREAKER_FAILURES_TOTAL.clone()))?;
        registry.register(Box::new(CIRCUIT_BREAKER_REJECTED_TOTAL.clone()))?;
        registry.register(Box::new(CIRCUIT_BREAKER_STATE_TRANSITIONS_TOTAL.clone()))?;

        // Error metrics
        registry.register(Box::new(ERROR_TOTAL.clone()))?;

        // Tenant cache metrics
        registry.register(Box::new(TENANT_CACHE_HITS.clone()))?;
        registry.register(Box::new(TENANT_CACHE_MISSES.clone()))?;

        let _ = REGISTRY.set(registry.clone());
        let _ = METRICS_HANDLE.set(handle.clone());
        Some(handle)
    } else {
        None
    };

    Ok(TelemetryHandles {
        metrics: metrics_handle,
    })
}

pub fn metrics_handle() -> Option<Arc<MetricsHandle>> {
    METRICS_HANDLE.get().cloned()
}

pub fn render_metrics() -> Result<String, prometheus::Error> {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.get()
        .ok_or(prometheus::Error::Msg("Registry not initialized".to_string()))?
        .gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer)?;
    Ok(String::from_utf8(buffer).unwrap_or_else(|_| String::from("Failed to encode metrics")))
}

pub fn current_trace_id() -> Option<String> {
    let span = tracing::Span::current();
    span.id().map(|id| id.into_u64().to_string())
}

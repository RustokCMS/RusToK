# OpenTelemetry Integration Guide

> **RusToK Sprint 3, Task 3.1**  
> Comprehensive OpenTelemetry integration for distributed tracing and observability.

---

## Overview

RusToK uses OpenTelemetry (OTel) for distributed tracing, providing visibility into request flows across the system.

### Features

- **Distributed Tracing**: Track requests across HTTP, GraphQL, and EventBus
- **OTLP Export**: Native gRPC export to Jaeger, Tempo, or any OTLP-compatible backend
- **Context Propagation**: Automatic trace context propagation
- **Configurable Sampling**: Adjust trace sampling rate via environment variables
- **Resource Attributes**: Service metadata attached to all spans

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      RusToK Server                          │
│                                                             │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐  │
│  │ HTTP Handler │───→│  GraphQL     │───→│  EventBus    │  │
│  │              │    │  Resolver    │    │              │  │
│  └──────┬───────┘    └──────┬───────┘    └──────┬───────┘  │
│         │                   │                    │          │
│         └───────────────────┼────────────────────┘          │
│                             │                               │
│                             ▼                               │
│                    ┌─────────────────┐                      │
│                    │  OpenTelemetry  │                      │
│                    │    Tracer       │                      │
│                    └────────┬────────┘                      │
└─────────────────────────────┼───────────────────────────────┘
                              │ OTLP gRPC
                              ▼
                    ┌─────────────────┐
                    │  Jaeger/Tempo   │
                    │   (Collector)   │
                    └────────┬────────┘
                              │
                              ▼
                    ┌─────────────────┐
                    │  Jaeger UI      │
                    │  (localhost:    │
                    │   16686)        │
                    └─────────────────┘
```

---

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `OTEL_SERVICE_NAME` | `rustok-server` | Service name in traces |
| `OTEL_SERVICE_VERSION` | `0.1.0` | Service version |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | `http://localhost:4317` | OTLP gRPC endpoint |
| `OTEL_SAMPLING_RATE` | `1.0` | Trace sampling rate (0.0-1.0) |
| `OTEL_ENABLED` | `true` | Enable/disable OpenTelemetry |
| `RUST_ENV` | `development` | Deployment environment |

### Example Configuration

```bash
# Development (default)
OTEL_ENABLED=true
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
OTEL_SAMPLING_RATE=1.0

# Production
OTEL_ENABLED=true
OTEL_EXPORTER_OTLP_ENDPOINT=https://otel-collector.mycompany.com:4317
OTEL_SAMPLING_RATE=0.1  # 10% sampling for high traffic
RUST_ENV=production
```

---

## Quick Start

### 1. Start Observability Stack

```bash
docker-compose -f docker-compose.observability.yml up -d
```

This starts:
- **Jaeger** on http://localhost:16686
- **Prometheus** on http://localhost:9090
- **Grafana** on http://localhost:3000

### 2. Run RusToK Server

```bash
# With default settings
cargo run -p rustok-server

# With custom OTLP endpoint
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317 \
  cargo run -p rustok-server
```

### 3. Generate Some Traffic

```bash
# Make some API calls
curl http://localhost:3000/api/health

# Or use the GraphQL playground
open http://localhost:3000/graphql
```

### 4. View Traces

Open Jaeger UI: http://localhost:16686

Select `rustok-server` from the Service dropdown and click "Find Traces".

---

## Usage in Code

### Automatic Instrumentation

RusToK automatically creates spans for:
- HTTP requests (via middleware)
- GraphQL resolvers
- EventBus operations
- Database queries

### Manual Span Creation

```rust
use tracing::{info_span, Instrument};

async fn process_order(order_id: Uuid) -> Result<(), Error> {
    let span = info_span!("process_order", order_id = %order_id);
    
    async move {
        // Your code here
        tracing::info!("Processing order...");
        
        // Nested spans
        let validate_span = info_span!("validate_order");
        validate_order().instrument(validate_span).await?;
        
        Ok(())
    }
    .instrument(span)
    .await
}
```

### Adding Custom Attributes

```rust
use tracing::info_span;

let span = info_span!(
    "create_content",
    tenant_id = %tenant_id,
    user_id = %user_id,
    content_type = "article"
);
```

### Using the traced_span! Macro

```rust
use rustok_telemetry::traced_span;

traced_span!("fetch_user", tenant_id = %tenant_id, user_id = %user_id, {
    let user = db.get_user(user_id).await?;
    Ok(user)
});
```

---

## Trace Context Propagation

### HTTP Headers

OpenTelemetry context is propagated via W3C Trace Context headers:

```
traceparent: 00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01
```

### EventBus Integration

Trace context is automatically propagated through events:

```rust
// When publishing an event
event_bus.publish(Event::UserCreated { user_id }).await?;

// The trace ID is automatically included in the event envelope
// and continued in the event handler
```

---

## Sampling Strategies

### Head-Based Sampling (Default)

Decides whether to sample at the start of a trace:

```bash
# Sample 10% of traces
OTEL_SAMPLING_RATE=0.1
```

### Custom Sampler

For more control, modify `otel.rs`:

```rust
use opentelemetry::sdk::trace::Sampler;

// Parent-based sampling
let sampler = Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(0.1)));

// Or use a custom function
let sampler = Sampler::from_env(); // OTEL_TRACES_SAMPLER env var
```

---

## Performance Considerations

### Overhead

| Component | Expected Overhead |
|-----------|-------------------|
| Span creation | ~1-5μs per span |
| Context propagation | ~100ns |
| OTLP export | Async, batched (minimal impact) |

### Optimization Tips

1. **Use sampling in production**: Set `OTEL_SAMPLING_RATE=0.1` or lower
2. **Batch configuration**: Already optimized (512 spans/batch, 5s interval)
3. **Filter spans**: Use `#[tracing::instrument(skip(...))]` to exclude large fields

### Batch Configuration

```rust
let batch_config = BatchConfig::default()
    .with_max_queue_size(2048)        // Max pending spans
    .with_max_export_batch_size(512)  // Spans per export
    .with_scheduled_delay(Duration::from_secs(5));  // Export interval
```

---

## Troubleshooting

### No Traces in Jaeger

1. Check Jaeger is running:
   ```bash
   docker ps | grep jaeger
   ```

2. Verify OTLP endpoint:
   ```bash
   curl http://localhost:16686/api/services
   ```

3. Check sampling rate:
   ```bash
   echo $OTEL_SAMPLING_RATE  # Should not be 0
   ```

4. Enable debug logging:
   ```bash
   RUST_LOG=debug cargo run -p rustok-server
   ```

### High Memory Usage

- Reduce `max_queue_size` in batch config
- Decrease `OTEL_SAMPLING_RATE`
- Check for span leaks (spans not being closed)

### Export Failures

```
ERROR opentelemetry_otlp::exporter::grpc: OTLP export failed: Connection refused
```

- Verify collector is running
- Check firewall rules
- Verify endpoint URL is correct

---

## Testing

### Unit Tests

```rust
#[tokio::test]
async fn test_tracing() {
    let config = OtelConfig {
        enabled: false,  // Disable export for tests
        ..Default::default()
    };
    
    init_tracing(config).await.unwrap();
    
    let span = info_span!("test_span");
    let _guard = span.enter();
    
    tracing::info!("Test message");
}
```

### Integration Tests

Run with a local Jaeger:

```bash
docker run -d --name jaeger \
  -p 16686:16686 \
  -p 4317:4317 \
  jaegertracing/all-in-one:1.52

cargo test -p rustok-telemetry -- --ignored
```

---

## Migration Guide

### From Basic Logging

Before:
```rust
log::info!("Processing order {}", order_id);
```

After:
```rust
tracing::info!(order_id = %order_id, "Processing order");
```

### Adding Tracing to Existing Code

```rust
#[tracing::instrument(skip(db), fields(order_id = %order_id))]
async fn process_order(db: &Database, order_id: Uuid) -> Result<()> {
    // Automatically creates a span on entry
    // Fields are automatically logged
}
```

---

## Best Practices

### Do's ✓

- Use semantic attribute names (`user_id`, `tenant_id`)
- Keep span names short and descriptive
- Use `#[tracing::instrument]` for automatic span creation
- Propagate context through all async boundaries
- Sample heavily in production (1-10%)

### Don'ts ✗

- Don't put sensitive data in span attributes
- Don't create too many spans (span per DB query is fine, per row is not)
- Don't ignore errors in span creation
- Don't use dynamic span names (hurts aggregation)

---

## Advanced Topics

### Custom Span Processors

```rust
use opentelemetry::sdk::trace::SpanProcessor;

struct CustomProcessor;

impl SpanProcessor for CustomProcessor {
    fn on_start(&self, span: &mut Span, cx: &Context) {
        // Custom logic on span start
    }
    
    fn on_end(&self, span: Span) {
        // Custom logic on span end
    }
    
    // ... other methods
}
```

### Multiple Exporters

```rust
// Export to both Jaeger and Zipkin
let jaeger_exporter = opentelemetry_jaeger::new_agent_pipeline()
    .with_endpoint("localhost:6831")
    .build_async_agent_exporter()
    .await?;

let zipkin_exporter = opentelemetry_zipkin::new_pipeline()
    .with_endpoint("http://localhost:9411/api/v2/spans")
    .init_exporter()?;
```

---

## References

- [OpenTelemetry Rust](https://docs.rs/opentelemetry/)
- [Jaeger Documentation](https://www.jaegertracing.io/docs/)
- [W3C Trace Context](https://www.w3.org/TR/trace-context/)
- [OpenTelemetry Semantic Conventions](https://opentelemetry.io/docs/concepts/semantic-conventions/)

---

## Support

For issues or questions:
- OpenTelemetry issues: Check [opentelemetry-rust](https://github.com/open-telemetry/opentelemetry-rust)
- RusToK issues: Create an issue in the RusToK repository

---

**Status**: ✅ Task 3.1 Complete  
**Last Updated**: 2026-02-12  
**Sprint**: 3 - Observability

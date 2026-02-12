# 📊 RusToK Metrics Dashboard Guide

> **Version:** 1.0
> **Updated:** 2026-02-12
> **Sprint:** 3 - Task 3.3

---

## 📋 Overview

This guide covers the metrics dashboard, custom Prometheus metrics, and alert rules for RusToK observability stack.

**Key Components:**
- ✅ Custom Prometheus metrics
- ✅ Grafana dashboards (Overview + Advanced)
- ✅ Alert rules for SLOs
- ✅ Docker Compose integration

---

## 🎯 Metrics Categories

### 1. HTTP Metrics

Monitor web request performance and errors.

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `rustok_http_requests_total` | Counter | `method`, `path`, `status` | Total HTTP requests |
| `rustok_http_request_duration_seconds` | Histogram | `method`, `path` | Request duration (with buckets) |

**Example Queries:**
```promql
# Request rate by status
sum(rate(rustok_http_requests_total[5m])) by (status)

# P95 latency
histogram_quantile(0.95, sum(rate(rustok_http_request_duration_seconds_bucket[5m])) by (le))

# Error rate
sum(rate(rustok_http_requests_total{status=~"5.."}[5m])) /
sum(rate(rustok_http_requests_total[5m]))
```

---

### 2. Content Module Metrics

Track content management operations.

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `rustok_content_operations_total` | Counter | `operation`, `kind`, `status` | Total content operations |
| `rustok_content_operation_duration_seconds` | Histogram | `operation`, `kind` | Operation duration |
| `rustok_content_nodes_total` | Gauge | - | Total number of content nodes |

**Example Queries:**
```promql
# Content operation rate by type
sum(rate(rustok_content_operations_total[5m])) by (operation, status)

# Total content nodes
rustok_content_nodes_total
```

---

### 3. Commerce Module Metrics

Monitor e-commerce operations.

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `rustok_commerce_operations_total` | Counter | `operation`, `kind`, `status` | Total commerce operations |
| `rustok_commerce_operation_duration_seconds` | Histogram | `operation`, `kind` | Operation duration |
| `rustok_commerce_products_total` | Gauge | - | Total number of products |
| `rustok_commerce_orders_total` | Gauge | - | Total number of orders |

**Example Queries:**
```promql
# Commerce operation rate
sum(rate(rustok_commerce_operations_total[5m])) by (operation)

# Total products
rustok_commerce_products_total
```

---

### 4. EventBus Metrics

Track event publishing and processing throughput.

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `rustok_eventbus_events_published_total` | Counter | - | Total events published |
| `rustok_eventbus_events_dropped_total` | Counter | - | Total events dropped |
| `rustok_eventbus_subscribers` | Gauge | - | Current number of subscribers |
| `rustok_eventbus_events_processed_total` | Counter | `module`, `status` | Total events processed |
| `rustok_eventbus_processing_duration_seconds` | Histogram | `module` | Event processing duration |

**Example Queries:**
```promql
# Event throughput
rate(rustok_eventbus_events_published_total[5m])

# Event drop rate
rate(rustok_eventbus_events_dropped_total[5m])

# Processing by module
sum(rate(rustok_eventbus_events_processed_total[5m])) by (module, status)

# Event lag (published - processed)
rate(rustok_eventbus_events_published_total[5m]) -
sum(rate(rustok_eventbus_events_processed_total[5m]))
```

---

### 5. Circuit Breaker Metrics

Monitor circuit breaker state and performance.

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `rustok_circuit_breaker_state` | Gauge | `name`, `service` | Current state (0=Closed, 1=Open, 2=HalfOpen) |
| `rustok_circuit_breaker_requests_total` | Counter | `name`, `service` | Total requests |
| `rustok_circuit_breaker_successes_total` | Counter | `name`, `service` | Total successes |
| `rustok_circuit_breaker_failures_total` | Counter | `name`, `service` | Total failures |
| `rustok_circuit_breaker_rejected_total` | Counter | `name`, `service` | Total rejected (fail-fast) |
| `rustok_circuit_breaker_state_transitions_total` | Counter | `name`, `service`, `from_state`, `to_state` | State transitions |

**Example Queries:**
```promql
# Circuit breaker state
rustok_circuit_breaker_state

# Rejection rate
rate(rustok_circuit_breaker_rejected_total[5m]) /
rate(rustok_circuit_breaker_requests_total[5m])

# State transitions
sum(rate(rustok_circuit_breaker_state_transitions_total[5m])) by (name, from_state, to_state)
```

---

### 6. Cache Metrics

Monitor cache performance and hit rates.

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `rustok_tenant_cache_hits` | Counter | - | Total cache hits |
| `rustok_tenant_cache_misses` | Counter | - | Total cache misses |

**Example Queries:**
```promql
# Cache hit rate
rate(rustok_tenant_cache_hits[5m]) /
(rate(rustok_tenant_cache_hits[5m]) + rate(rustok_tenant_cache_misses[5m]))

# Miss rate
rate(rustok_tenant_cache_misses[5m]) /
(rate(rustok_tenant_cache_hits[5m]) + rate(rustok_tenant_cache_misses[5m]))
```

---

### 7. Error Metrics

Track errors across modules.

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `rustok_errors_total` | Counter | `module`, `error_type`, `severity` | Total errors |

**Example Queries:**
```promql
# Error rate by module
sum(rate(rustok_errors_total[5m])) by (module, severity)

# Critical errors
sum(rate(rustok_errors_total{severity="critical"}[5m])) by (module)

# Error rate percentage
sum(rate(rustok_errors_total[5m])) /
sum(rate(rustok_http_requests_total[5m]))
```

---

## 🎨 Grafana Dashboards

### Dashboard 1: RusToK Overview

**Location:** `grafana/dashboards/rustok-overview.json`

**Panels:**
1. HTTP Request Rate
2. HTTP Request Duration (P95)
3. HTTP Requests by Status (pie chart)
4. Content Operations Rate
5. Commerce Operations Rate
6. Total Content Nodes
7. Total Products

**Access:** http://localhost:3000/d/rustok-overview

---

### Dashboard 2: RusToK Advanced

**Location:** `grafana/dashboards/rustok-advanced.json`

**Panels:**
1. Error Rate (gauge)
2. HTTP Request Rate (by method)
3. P50 Latency (gauge)
4. P95 Latency (gauge)
5. P99 Latency (gauge)
6. Requests by Status (pie chart)
7. Circuit Breaker State (color-coded)
8. Circuit Breaker Rejection Rate
9. EventBus Throughput
10. EventBus Subscribers
11. Error Rate by Module
12. Business Operations
13. Cache Hit Rate
14. Event Processing by Module

**Access:** http://localhost:3000/d/rustok-advanced

---

## 🚨 Alert Rules

### Alert Categories

#### 1. Critical Alerts (Severity: Critical)

| Alert | Condition | Duration | Description |
|-------|-----------|----------|-------------|
| `HighErrorRate` | Error rate > 5% | 5m | High error rate detected |
| `SlowRequestsP99` | P99 > 1s | 5m | Very slow requests |
| `CircuitBreakerOpen` | State = Open | 2m | Circuit breaker is open |
| `CriticalModuleErrorRate` | Critical errors > 20/s | 2m | Critical error rate |
| `ServiceNotResponding` | Service down | 1m | Server not responding |
| `SLOAvailabilityBudgetExhausted` | Error rate > 0.1% | 10m | SLO availability breached |

#### 2. Warning Alerts (Severity: Warning)

| Alert | Condition | Duration | Description |
|-------|-----------|----------|-------------|
| `SlowRequestsP95` | P95 > 500ms | 5m | Slow requests detected |
| `CircuitBreakerHighRejectionRate` | Rejection > 50% | 5m | High rejection rate |
| `EventBusEventsDropped` | Drop rate > 10/s | 2m | Events dropping |
| `LowCacheHitRate` | Hit rate < 50% | 10m | Low cache performance |
| `HighModuleErrorRate` | Critical errors > 5/s | 5m | High error rate |
| `SLOLatencyP95Breached` | P95 > 500ms | 10m | SLO latency breached |
| `SLOLatencyP99Breached` | P99 > 1s | 10m | SLO latency breached |

#### 3. Info Alerts (Severity: Info)

| Alert | Condition | Duration | Description |
|-------|-----------|----------|-------------|
| `EventBusNoSubscribers` | Subscribers = 0 | 5m | No event subscribers |

---

### Alert Configuration

**Location:** `prometheus/alerts/rustok-alerts.yml`

**Key SLOs:**

| SLO Type | Target | Metric | Alert |
|----------|--------|--------|-------|
| Availability | 99.9% | Error rate < 0.1% | SLOAvailabilityBudgetExhausted |
| Latency P95 | < 500ms | P95 latency | SLOLatencyP95Breached |
| Latency P99 | < 1s | P99 latency | SLOLatencyP99Breached |

---

## 🚀 Quick Start

### 1. Start Observability Stack

```bash
# Start all services
docker-compose -f docker-compose.observability.yml up -d

# Check status
docker-compose -f docker-compose.observability.yml ps
```

### 2. Run Server with Metrics

```bash
# Set environment variables
export RUSTOK_METRICS_ENABLED=true

# Run server
cargo run -p rustok-server
```

### 3. Access Dashboards

- **Grafana:** http://localhost:3000 (admin/admin)
- **Prometheus:** http://localhost:9090
- **Jaeger:** http://localhost:16686

### 4. Check Metrics Endpoint

```bash
# Check metrics endpoint
curl http://localhost:3000/api/_health/metrics

# Check specific metrics
curl http://localhost:3000/api/_health/metrics | grep rustok_http_requests_total
```

### 5. View Alerts

```bash
# Check alert rules in Prometheus UI
# http://localhost:9090/rules

# Check active alerts
# http://localhost:9090/alerts
```

---

## 📊 Common Queries

### Error Analysis

```promql
# Top 10 error types
topk(10, sum(rate(rustok_errors_total[1h])) by (error_type))

# Error rate trend
sum(rate(rustok_errors_total[5m])) by (severity)

# HTTP 5xx error rate
sum(rate(rustok_http_requests_total{status=~"5.."}[5m])) by (path)
```

### Performance Analysis

```promql
# Average response time by endpoint
rate(rustok_http_request_duration_seconds_sum[5m]) /
rate(rustok_http_request_duration_seconds_count[5m])

# Slowest endpoints
topk(5,
  histogram_quantile(0.95,
    sum(rate(rustok_http_request_duration_seconds_bucket[5m])) by (le, path)
  )
)

# Request rate by path
sum(rate(rustok_http_requests_total[5m])) by (path)
```

### Circuit Breaker Health

```promql
# All circuit breakers
rustok_circuit_breaker_state

# Circuit breakers with high rejection rate
rate(rustok_circuit_breaker_rejected_total[5m]) /
rate(rustok_circuit_breaker_requests_total[5m]) > 0.1

# Circuit breaker state transitions
sum(increase(rustok_circuit_breaker_state_transitions_total[1h])) by (name, to_state)
```

### EventBus Analysis

```promql
# Event processing lag
rate(rustok_eventbus_events_published_total[5m]) -
sum(rate(rustok_eventbus_events_processed_total[5m]))

# Events dropped percentage
rate(rustok_eventbus_events_dropped_total[5m]) /
rate(rustok_eventbus_events_published_total[5m])

# Slowest event processors
topk(5,
  histogram_quantile(0.95,
    sum(rate(rustok_eventbus_processing_duration_seconds_bucket[5m])) by (le, module)
  )
)
```

---

## 🔧 Configuration

### Prometheus Configuration

**Location:** `prometheus/prometheus.yml`

**Scrape Intervals:**
- Default: 15 seconds
- Evaluation: 15 seconds
- Retention: 30 days

**Customization:**
```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

  # Add custom labels
  external_labels:
    cluster: 'rustok-prod'
    environment: 'production'
```

### Alert Rules Configuration

**Location:** `prometheus/alerts/rustok-alerts.yml`

**Modify thresholds:**
```yaml
# Example: Change error rate threshold
- alert: HighErrorRate
  expr: |
    sum(rate(rustok_http_requests_total{status=~"5.."}[5m])) /
    sum(rate(rustok_http_requests_total[5m])) > 0.10  # Changed from 0.05
  for: 5m
  labels:
    severity: critical
```

### Grafana Dashboard Configuration

**Edit dashboard:**
1. Open Grafana
2. Navigate to dashboard
3. Click "Settings" (gear icon)
4. Edit JSON
5. Save changes

**Dashboard variables:**
Add variables for dynamic filtering (e.g., environment, tenant_id).

---

## 🧪 Testing Metrics

### 1. Verify Metrics Endpoint

```bash
# Check endpoint returns data
curl -s http://localhost:3000/api/_health/metrics | head -20

# Check specific metric exists
curl -s http://localhost:3000/api/_health/metrics | grep rustok_http_requests_total
```

### 2. Test Alert Rules

```bash
# Check rules loaded in Prometheus
curl -s http://localhost:3000/api/v1/rules | jq '.data.groups[] | select(.name=="rustok_alerts")'

# Check alert states
curl -s http://localhost:3000/api/v1/alerts | jq '.data.alerts[]'
```

### 3. Simulate Load

```bash
# Generate requests
for i in {1..100}; do
  curl -s http://localhost:3000/api/content/nodes > /dev/null
done

# Check metrics updated
curl -s http://localhost:3000/api/_health/metrics | grep rustok_http_requests_total
```

---

## 📈 Best Practices

### 1. Metric Naming

- Use `rustok_` prefix for all metrics
- Use descriptive names (e.g., `http_requests_total` not `req_cnt`)
- Include units in metric names (e.g., `duration_seconds`)
- Use `_total` suffix for counters
- Use `_bytes` or `_seconds` for gauges with units

### 2. Label Strategy

- Use low-cardinality labels (< 50 unique values)
- Avoid high-cardinality labels (e.g., user_id)
- Include common labels: `module`, `status`, `severity`
- Use consistent label values across metrics

### 3. Query Optimization

- Use `rate()` for counters over time windows
- Use `histogram_quantile()` for latency percentiles
- Use `sum()` with `by()` for aggregation
- Avoid complex queries in dashboards (precompute if needed)

### 4. Alert Configuration

- Set appropriate `for` duration (avoid alert flapping)
- Use severity levels (critical, warning, info)
- Include clear annotations (summary + description)
- Test alerts before production deployment

### 5. SLO Monitoring

- Define clear SLO targets (availability, latency)
- Set error budgets (e.g., 0.1% for 99.9% availability)
- Create separate alert groups for SLO breaches
- Regularly review and adjust SLOs

---

## 🐛 Troubleshooting

### Metrics Not Showing

**Symptom:** No metrics in Prometheus

**Solutions:**
1. Check server is running: `curl http://localhost:3000/api/_health/metrics`
2. Verify Prometheus scraping: http://localhost:9090/targets
3. Check firewall rules (port 3000 accessible)
4. Review server logs for metrics initialization errors

### Alerts Not Firing

**Symptom:** Expected alerts not triggered

**Solutions:**
1. Check alert rules loaded: http://localhost:9090/rules
2. Verify `for` duration conditions met
3. Check alert state transitions in UI
4. Review Prometheus logs for evaluation errors

### Dashboard Not Loading

**Symptom:** Grafana dashboard shows "No data"

**Solutions:**
1. Check Prometheus datasource connectivity
2. Verify metric names match (case-sensitive)
3. Check time range (default "Last 1 hour")
4. Review dashboard JSON for syntax errors

### High Memory Usage

**Symptom:** Prometheus using excessive memory

**Solutions:**
1. Reduce retention time: `--storage.tsdb.retention.time=15d`
2. Increase scrape interval: `scrape_interval: 30s`
3. Reduce number of time series (remove high-cardinality labels)
4. Add memory limits to docker-compose

---

## 📚 References

### Internal Documentation
- [OBSERVABILITY_QUICKSTART.md](../OBSERVABILITY_QUICKSTART.md)
- [DISTRIBUTED_TRACING_GUIDE.md](./DISTRIBUTED_TRACING_GUIDE.md)
- [SPRINT_3_START.md](../SPRINT_3_START.md)

### External Resources
- [Prometheus Documentation](https://prometheus.io/docs/)
- [Grafana Documentation](https://grafana.com/docs/)
- [PromQL Query Language](https://prometheus.io/docs/prometheus/latest/querying/basics/)
- [Alerting Rules](https://prometheus.io/docs/prometheus/latest/configuration/alerting_rules/)

---

## 📞 Support

**Issues:** Report bugs or feature requests via GitHub Issues

**Documentation:** This guide is maintained in `docs/METRICS_DASHBOARD_GUIDE.md`

---

**Version:** 1.0
**Last Updated:** 2026-02-12
**Maintained By:** RusToK Team

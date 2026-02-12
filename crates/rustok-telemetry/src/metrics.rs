//! Metrics helpers and integration utilities
//!
//! This module provides helper functions and utilities for integrating
//! custom metrics across the RusToK system.

use std::sync::Arc;
use std::time::Instant;
use tracing::instrument;

use super::lib::{
    EVENTBUS_EVENTS_PUBLISHED_TOTAL,
    EVENTBUS_EVENTS_DROPPED_TOTAL,
    EVENTBUS_SUBSCRIBERS,
    EVENTBUS_EVENTS_PROCESSED_TOTAL,
    EVENTBUS_PROCESSING_DURATION_SECONDS,
    CIRCUIT_BREAKER_STATE,
    CIRCUIT_BREAKER_REQUESTS_TOTAL,
    CIRCUIT_BREAKER_SUCCESSES_TOTAL,
    CIRCUIT_BREAKER_FAILURES_TOTAL,
    CIRCUIT_BREAKER_REJECTED_TOTAL,
    CIRCUIT_BREAKER_STATE_TRANSITIONS_TOTAL,
    ERROR_TOTAL,
    TENANT_CACHE_HITS,
    TENANT_CACHE_MISSES,
    CONTENT_OPERATIONS_TOTAL,
    CONTENT_OPERATION_DURATION_SECONDS,
    COMMERCE_OPERATIONS_TOTAL,
    COMMERCE_OPERATION_DURATION_SECONDS,
    HTTP_REQUESTS_TOTAL,
    HTTP_REQUEST_DURATION_SECONDS,
};

/// Metrics guard for automatic timing
pub struct MetricsTimer {
    start: Instant,
    labels: Vec<String>,
}

impl MetricsTimer {
    pub fn new(labels: Vec<String>) -> Self {
        Self {
            start: Instant::now(),
            labels,
        }
    }

    pub fn duration(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
}

/// Event processing timer
pub struct EventProcessingTimer {
    start: Instant,
    module: String,
}

impl EventProcessingTimer {
    pub fn new(module: String) -> Self {
        Self {
            start: Instant::now(),
            module,
        }
    }

    pub fn record_success(&self) {
        let duration = self.start.elapsed().as_secs_f64();
        EVENTBUS_EVENTS_PROCESSED_TOTAL
            .with_label_values(&[&self.module, "success"])
            .inc();
        EVENTBUS_PROCESSING_DURATION_SECONDS
            .with_label_values(&[&self.module])
            .observe(duration);
    }

    pub fn record_failure(&self) {
        let duration = self.start.elapsed().as_secs_f64();
        EVENTBUS_EVENTS_PROCESSED_TOTAL
            .with_label_values(&[&self.module, "failure"])
            .inc();
        EVENTBUS_PROCESSING_DURATION_SECONDS
            .with_label_values(&[&self.module])
            .observe(duration);
    }
}

impl Drop for EventProcessingTimer {
    fn drop(&mut self) {
        // Record duration if not already recorded
        let duration = self.start.elapsed().as_secs_f64();
        EVENTBUS_PROCESSING_DURATION_SECONDS
            .with_label_values(&[&self.module])
            .observe(duration);
    }
}

/// HTTP request timer
pub struct HttpRequestTimer {
    start: Instant,
    method: String,
    path: String,
}

impl HttpRequestTimer {
    pub fn new(method: String, path: String) -> Self {
        Self {
            start: Instant::now(),
            method,
            path,
        }
    }

    pub fn record(&self, status: u16) {
        let duration = self.start.elapsed().as_secs_f64();
        let status_str = status.to_string();

        HTTP_REQUESTS_TOTAL
            .with_label_values(&[&self.method, &self.path, &status_str])
            .inc();
        HTTP_REQUEST_DURATION_SECONDS
            .with_label_values(&[&self.method, &self.path])
            .observe(duration);
    }
}

/// Content operation timer
pub struct ContentOperationTimer {
    start: Instant,
    operation: String,
    kind: String,
}

impl ContentOperationTimer {
    pub fn new(operation: String, kind: String) -> Self {
        Self {
            start: Instant::now(),
            operation,
            kind,
        }
    }

    pub fn record_success(&self) {
        let duration = self.start.elapsed().as_secs_f64();
        CONTENT_OPERATIONS_TOTAL
            .with_label_values(&[&self.operation, &self.kind, "success"])
            .inc();
        CONTENT_OPERATION_DURATION_SECONDS
            .with_label_values(&[&self.operation, &self.kind])
            .observe(duration);
    }

    pub fn record_failure(&self) {
        let duration = self.start.elapsed().as_secs_f64();
        CONTENT_OPERATIONS_TOTAL
            .with_label_values(&[&self.operation, &self.kind, "failure"])
            .inc();
        CONTENT_OPERATION_DURATION_SECONDS
            .with_label_values(&[&self.operation, &self.kind])
            .observe(duration);
    }
}

/// Commerce operation timer
pub struct CommerceOperationTimer {
    start: Instant,
    operation: String,
    kind: String,
}

impl CommerceOperationTimer {
    pub fn new(operation: String, kind: String) -> Self {
        Self {
            start: Instant::now(),
            operation,
            kind,
        }
    }

    pub fn record_success(&self) {
        let duration = self.start.elapsed().as_secs_f64();
        COMMERCE_OPERATIONS_TOTAL
            .with_label_values(&[&self.operation, &self.kind, "success"])
            .inc();
        COMMERCE_OPERATION_DURATION_SECONDS
            .with_label_values(&[&self.operation, &self.kind])
            .observe(duration);
    }

    pub fn record_failure(&self) {
        let duration = self.start.elapsed().as_secs_f64();
        COMMERCE_OPERATIONS_TOTAL
            .with_label_values(&[&self.operation, &self.kind, "failure"])
            .inc();
        COMMERCE_OPERATION_DURATION_SECONDS
            .with_label_values(&[&self.operation, &self.kind])
            .observe(duration);
    }
}

/// Record EventBus metrics
pub fn record_eventbus_published() {
    EVENTBUS_EVENTS_PUBLISHED_TOTAL.inc();
}

pub fn record_eventbus_dropped() {
    EVENTBUS_EVENTS_DROPPED_TOTAL.inc();
}

pub fn record_eventbus_subscriber_count(count: usize) {
    EVENTBUS_SUBSCRIBERS.set(count as i64);
}

/// Record circuit breaker metrics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitBreakerState {
    Closed = 0,
    Open = 1,
    HalfOpen = 2,
}

pub fn record_circuit_breaker_state(name: &str, service: &str, state: CircuitBreakerState) {
    CIRCUIT_BREAKER_STATE
        .with_label_values(&[name, service])
        .set(state as i64);
}

pub fn record_circuit_breaker_request(name: &str, service: &str) {
    CIRCUIT_BREAKER_REQUESTS_TOTAL
        .with_label_values(&[name, service])
        .inc();
}

pub fn record_circuit_breaker_success(name: &str, service: &str) {
    CIRCUIT_BREAKER_SUCCESSES_TOTAL
        .with_label_values(&[name, service])
        .inc();
}

pub fn record_circuit_breaker_failure(name: &str, service: &str) {
    CIRCUIT_BREAKER_FAILURES_TOTAL
        .with_label_values(&[name, service])
        .inc();
}

pub fn record_circuit_breaker_rejected(name: &str, service: &str) {
    CIRCUIT_BREAKER_REJECTED_TOTAL
        .with_label_values(&[name, service])
        .inc();
}

pub fn record_circuit_breaker_state_transition(
    name: &str,
    service: &str,
    from: CircuitBreakerState,
    to: CircuitBreakerState,
) {
    CIRCUIT_BREAKER_STATE_TRANSITIONS_TOTAL
        .with_label_values(&[name, service, &format!("{:?}", from), &format!("{:?}", to)])
        .inc();
}

/// Record error metrics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    Info = 0,
    Warning = 1,
    Error = 2,
    Critical = 3,
}

pub fn record_error(module: &str, error_type: &str, severity: ErrorSeverity) {
    ERROR_TOTAL
        .with_label_values(&[module, error_type, &format!("{:?}", severity)])
        .inc();
}

/// Record cache metrics
pub fn record_cache_hit() {
    TENANT_CACHE_HITS.inc();
}

pub fn record_cache_miss() {
    TENANT_CACHE_MISSES.inc();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_timer_duration() {
        let timer = MetricsTimer::new(vec!["test".to_string()]);
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(timer.duration() >= 0.01);
    }

    #[test]
    fn test_event_processing_timer() {
        let timer = EventProcessingTimer::new("test_module".to_string());
        std::thread::sleep(std::time::Duration::from_millis(10));
        timer.record_success();
        // Verify metrics were recorded (would need to check Prometheus registry)
    }

    #[test]
    fn test_circuit_breaker_state_values() {
        assert_eq!(CircuitBreakerState::Closed as i64, 0);
        assert_eq!(CircuitBreakerState::Open as i64, 1);
        assert_eq!(CircuitBreakerState::HalfOpen as i64, 2);
    }

    #[test]
    fn test_error_severity_values() {
        assert_eq!(ErrorSeverity::Info as i64, 0);
        assert_eq!(ErrorSeverity::Warning as i64, 1);
        assert_eq!(ErrorSeverity::Error as i64, 2);
        assert_eq!(ErrorSeverity::Critical as i64, 3);
    }
}

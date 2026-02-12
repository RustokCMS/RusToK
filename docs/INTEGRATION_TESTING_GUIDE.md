# Integration Testing Guide

> **Last Updated:** 2026-02-12
> **Status:** ✅ Active
> **Maintainer:** RusToK Team

---

## Table of Contents

1. [Overview](#overview)
2. [Test Utilities Framework](#test-utilities-framework)
3. [Running Integration Tests](#running-integration-tests)
4. [Writing Integration Tests](#writing-integration-tests)
5. [Test Fixtures](#test-fixtures)
6. [Event Testing](#event-testing)
7. [Mock Services](#mock-services)
8. [CI/CD Integration](#cicd-integration)
9. [Best Practices](#best-practices)
10. [Troubleshooting](#troubleshooting)

---

## Overview

Integration tests in RusToK verify that different components work together correctly. They test:
- Complete business workflows (order flow, content flow)
- Event propagation and handling
- API endpoints end-to-end
- Database operations and transactions
- Multi-tenant isolation

### Key Features

- **Test Utilities Crate**: Reusable test fixtures and helpers
- **Test App Wrapper**: HTTP client for API testing
- **Event Capture**: Verify events are emitted correctly
- **Deterministic Data**: UUID generators and test data fixtures
- **Database Isolation**: Each test runs in a clean database state

---

## Test Utilities Framework

The `rustok-test-utils` crate provides utilities for integration testing.

### Structure

```
crates/rustok-test-utils/
├── src/
│   ├── lib.rs           # Main exports
│   ├── fixtures.rs      # Test data generators
│   ├── test_app.rs      # Test application wrapper
│   ├── db.rs            # Database helpers
│   ├── events.rs        # Event helpers
│   └── helpers.rs       # Misc helpers
└── Cargo.toml
```

### Dependencies

Add to your integration test's `Cargo.toml`:

```toml
[dev-dependencies]
rustok-test-utils = { path = "../../crates/rustok-test-utils" }
tokio = { version = "1", features = ["full"] }
```

### Using Test Utilities

```rust
use rustok_test_utils::*;

#[tokio::test]
async fn my_integration_test() {
    let app = spawn_test_app().await;
    
    // Use the app to make API calls
    let product = app.create_product(test_product_input())
        .await
        .expect("Failed to create product");
    
    assert_eq!(product.sku, "TEST-001");
}
```

---

## Running Integration Tests

### Prerequisites

1. **Start PostgreSQL** (test database)

```bash
# Using Docker
docker run -d --name rustok-test-db \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=rustok_test \
  -p 5432:5432 \
  postgres:16
```

2. **Run Database Migrations**

```bash
cd apps/server
cargo loco db migrate
```

3. **Start Test Server** (in separate terminal)

```bash
cd apps/server
TEST_MODE=1 cargo loco start
```

### Running All Integration Tests

```bash
# Run all tests (ignored tests require server)
cargo test --test '*' --workspace -- --ignored

# Run only integration tests
cargo test --test '*' --workspace --test-threads=1 -- --ignored integration

# Run specific test suite
cargo test --test order_flow_test --workspace -- --ignored
```

### Environment Variables

Integration tests use these environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `TEST_DATABASE_URL` | `postgres://postgres:password@localhost:5432/rustok_test` | Test database URL |
| `TEST_SERVER_URL` | `http://localhost:3000` | Server base URL |
| `TEST_AUTH_TOKEN` | `test_token` | Authentication token |
| `TEST_TENANT_ID` | `test-tenant` | Default tenant ID |
| `TEST_USER_ID` | auto-generated | Default user ID |

---

## Writing Integration Tests

### Test Structure

```rust
//! # My Integration Test
//!
//! Description of what this test verifies

use rustok_test_utils::*;

/// Test description
#[tokio::test]
#[ignore] // Requires server to be running
async fn test_my_feature() {
    // 1. Setup
    let app = spawn_test_app().await;
    
    // 2. Execute
    let result = app.do_something().await;
    
    // 3. Verify
    assert!(result.is_ok());
    
    // 4. Cleanup (automatic)
}
```

### Common Patterns

#### 1. Create and Verify

```rust
#[tokio::test]
#[ignore]
async fn test_create_and_retrieve() {
    let app = spawn_test_app().await;
    
    // Create
    let created = app.create_node(test_node_input())
        .await
        .expect("Failed to create node");
    
    // Retrieve
    let retrieved = app.get_node(created.id)
        .await
        .expect("Failed to retrieve node");
    
    // Verify
    assert_eq!(created.id, retrieved.id);
    assert_eq!(created.title, retrieved.title);
}
```

#### 2. State Transitions

```rust
#[tokio::test]
#[ignore]
async fn test_state_transition() {
    let app = spawn_test_app().await;
    
    let node = app.create_node(test_node_input()).await.unwrap();
    assert_eq!(node.status, "draft");
    
    let published = app.publish_node(node.id).await.unwrap();
    assert_eq!(published.status, "published");
    
    // Verify events
    let events = app.get_events_for_node(node.id).await;
    assert!(events.iter().any(|e| matches!(e, DomainEvent::NodePublished { .. })));
}
```

#### 3. Error Handling

```rust
#[tokio::test]
#[ignore]
async fn test_validation_error() {
    let app = spawn_test_app().await;
    
    let result = app.create_node(CreateNodeInput {
        title: "", // Invalid: empty title
        ..Default::default()
    }).await;
    
    assert!(result.is_err());
    
    match result {
        Err(TestAppError::ApiError { status, message }) => {
            assert_eq!(status, 400);
            assert!(message.contains("title"));
        }
        _ => panic!("Expected API error"),
    }
}
```

#### 4. Search and Filtering

```rust
#[tokio::test]
#[ignore]
async fn test_search() {
    let app = spawn_test_app().await;
    
    // Create test data
    app.create_node(test_node_with_title("Rust Programming")).await.unwrap();
    app.create_node(test_node_with_title("Python Guide")).await.unwrap();
    app.create_node(test_node_with_title("Rust Best Practices")).await.unwrap();
    
    // Search
    let results = app.search_nodes("Rust").await.unwrap();
    
    assert_eq!(results.len(), 2);
}
```

---

## Test Fixtures

### Fixtures Module

The `fixtures` module provides generators for test data:

```rust
use rustok_test_utils::*;

// ID generators
let id = test_uuid();
let customer_id = test_customer_id();

// Domain fixtures
let product_input = test_product_input();
let node_input = test_node_input();
let order_input = test_order_input();

// With custom values
let custom_product = test_product_input_with_sku("CUSTOM-001");
```

### Available Fixtures

#### UUID Generators

```rust
pub fn test_uuid() -> Uuid
pub fn test_customer_id() -> Uuid
pub fn test_user_id() -> Uuid
```

#### Content Fixtures

```rust
pub fn test_node_input() -> CreateNodeInput
pub fn test_node_with_title(title: &str) -> CreateNodeInput
pub fn test_translation_input(locale: &str) -> TranslationInput
```

#### Commerce Fixtures

```rust
pub fn test_product_input() -> CreateProductInput
pub fn test_product_input_with_sku(sku: &str) -> CreateProductInput
pub fn test_order_input() -> CreateOrderInput
pub fn test_order_item_input() -> OrderItemInput
pub fn test_payment_input() -> ProcessPaymentInput
```

#### Event Fixtures

```rust
pub fn test_node_created_event() -> DomainEvent
pub fn test_order_created_event() -> DomainEvent
```

---

## Event Testing

### Verifying Events

```rust
#[tokio::test]
#[ignore]
async fn test_events_emitted() {
    let app = spawn_test_app().await;
    
    // Trigger event
    let node = app.create_node(test_node_input()).await.unwrap();
    
    // Verify event
    let events = app.get_events_for_node(node.id).await;
    
    assert!(!events.is_empty());
    assert!(events.iter().any(|e| matches!(e, DomainEvent::NodeCreated { .. })));
}
```

### Event Ordering

```rust
#[tokio::test]
#[ignore]
async fn test_event_ordering() {
    let app = spawn_test_app().await;
    
    let node = app.create_node(test_node_input()).await.unwrap();
    let _published = app.publish_node(node.id).await.unwrap();
    
    let events = app.get_events_for_node(node.id).await;
    
    // Verify order
    let first = &events[0];
    let second = &events[1];
    
    assert!(matches!(first, DomainEvent::NodeCreated { .. }));
    assert!(matches!(second, DomainEvent::NodePublished { .. }));
}
```

### Event Correlation

```rust
#[tokio::test]
#[ignore]
async fn test_event_correlation() {
    let app = spawn_test_app().await;
    
    let node = app.create_node(test_node_input()).await.unwrap();
    
    let events = app.get_events_for_node(node.id).await;
    
    // All events should have same node_id
    for event in events {
        let node_id = match event {
            DomainEvent::NodeCreated { node_id, .. } => *node_id,
            DomainEvent::NodePublished { node_id, .. } => *node_id,
            _ => continue,
        };
        assert_eq!(node_id, node.id);
    }
}
```

---

## Mock Services

### Payment Gateway Mock

For integration tests that need payment processing, use a mock payment gateway:

```rust
// In test configuration
#[cfg(test)]
pub mod mock_payment {
    use serde::{Serialize, Deserialize};
    
    #[derive(Serialize, Deserialize)]
    pub struct MockPaymentRequest {
        pub card_token: String,
        pub amount: i64,
    }
    
    #[derive(Serialize, Deserialize)]
    pub struct MockPaymentResponse {
        pub success: bool,
        pub payment_id: String,
    }
    
    pub fn mock_process_payment(req: MockPaymentRequest) -> MockPaymentResponse {
        // Accept all test tokens starting with "tok_test"
        let success = req.card_token.starts_with("tok_test");
        
        MockPaymentResponse {
            success,
            payment_id: if success {
                format!("pay_mock_{}", uuid::Uuid::new_v4())
            } else {
                "failed".to_string()
            },
        }
    }
}

// Usage in test
let response = mock_process_payment(MockPaymentRequest {
    card_token: "tok_test_visa".to_string(),
    amount: 1000,
});

assert!(response.success);
```

### External API Mock

For tests that interact with external APIs:

```rust
// Use wiremock for HTTP mocking
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
#[ignore]
async fn test_with_external_api() {
    // Start mock server
    let mock_server = MockServer::start().await;
    
    // Setup mock response
    Mock::given(method("GET"))
        .and(path("/api/data"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "data": "test"
        })))
        .mount(&mock_server)
        .await;
    
    // Configure app to use mock server
    let mut app = spawn_test_app().await;
    app.base_url = mock_server.uri();
    
    // Run test...
}
```

---

## CI/CD Integration

### GitHub Actions Workflow

Integration tests are run in CI/CD via the `integration-tests` job in `.github/workflows/ci.yml`:

```yaml
integration-tests:
  name: Integration Tests
  runs-on: ubuntu-latest
  needs: build-server
  services:
    postgres:
      image: postgres:16
      env:
        POSTGRES_USER: postgres
        POSTGRES_PASSWORD: postgres
        POSTGRES_DB: rustok_test
      ports:
        - 5432:5432
      options: >-
        --health-cmd pg_isready
        --health-interval 10s
        --health-timeout 5s
        --health-retries 5
  env:
    DATABASE_URL: postgres://postgres:postgres@localhost:5432/rustok_test
    TEST_SERVER_URL: http://localhost:3000
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - uses: Swatinem/rust-cache@v2
    
    - name: Run migrations
      run: |
        cd apps/server
        cargo loco db migrate
    
    - name: Start test server
      run: |
        cd apps/server
        TEST_MODE=1 cargo loco start &
        sleep 30  # Wait for server to be ready
    
    - name: Run integration tests
      run: |
        cargo test --test '*' --workspace -- --ignored --test-threads=1
```

### Running in CI

Integration tests in CI:
1. Start PostgreSQL service
2. Run database migrations
3. Start test server in background
4. Run all `#[ignore]` tests
5. Collect results

---

## Best Practices

### 1. Test Isolation

Each test should be independent:

```rust
#[tokio::test]
#[ignore]
async fn test_isolated() {
    let app = spawn_test_app().await;
    
    // Don't rely on state from other tests
    // Create all data needed in this test
    let product = app.create_product(test_product_input())
        .await
        .expect("Failed to create product");
    
    // Test...
}
```

### 2. Deterministic Data

Use deterministic fixtures:

```rust
// GOOD: Deterministic
let id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();

// OK: Generated but consistent for test
let id = test_uuid(); // Uses seed-based generator

// AVOID: Random
let id = Uuid::new_v4(); // Different every time
```

### 3. Explicit Assertions

Be explicit about what you're testing:

```rust
// GOOD: Clear and specific
assert_eq!(order.status, OrderStatus::Paid.to_string());
assert_eq!(order.total, 3000);

// AVOID: Generic
assert!(order.is_ok());
```

### 4. Test Lifecycle

Follow the Arrange-Act-Assert pattern:

```rust
#[tokio::test]
#[ignore]
async fn test_aaa_pattern() {
    // Arrange: Setup test data
    let app = spawn_test_app().await;
    let product = app.create_product(test_product_input()).await.unwrap();
    
    // Act: Execute the feature
    let order = app.create_order(CreateOrderInput {
        customer_id: test_customer_id(),
        items: vec![OrderItemInput {
            product_id: product.id,
            quantity: 1,
            price: Some(1000),
        }],
    }).await.unwrap();
    
    // Assert: Verify results
    assert_eq!(order.status, OrderStatus::Draft.to_string());
    assert_eq!(order.total, 1000);
}
```

### 5. Error Messages

Provide helpful error messages:

```rust
// GOOD: Clear error message
assert_eq!(order.status, OrderStatus::Paid.to_string(), 
    "Order should be in Paid status after payment, got: {}", order.status);

// AVOID: Generic message
assert_eq!(order.status, OrderStatus::Paid.to_string());
```

### 6. Test Data Management

Clean up test data:

```rust
#[tokio::test]
#[ignore]
async fn test_with_cleanup() {
    let app = spawn_test_app().await;
    
    // Create data
    let node = app.create_node(test_node_input()).await.unwrap();
    
    // Test...
    
    // Cleanup (optional if using transactions)
    // Database rollback happens automatically in most cases
}
```

### 7. Asynchronous Testing

Use proper async/await:

```rust
// GOOD: Proper async
#[tokio::test]
#[ignore]
async fn test_async() {
    let app = spawn_test_app().await;
    let result = app.get_node(test_uuid()).await;
    // ...
}

// AVOID: Blocking
#[test]
fn test_sync() {
    let app = spawn_test_app().await.unwrap(); // Don't unwrap futures
    // ...
}
```

---

## Troubleshooting

### Test Server Won't Start

**Problem:** `Connection refused` when running integration tests

**Solution:**
1. Check if server is running: `curl http://localhost:3000/health`
2. Start server manually: `cd apps/server && TEST_MODE=1 cargo loco start`
3. Check port availability: `lsof -i :3000`

### Database Connection Failed

**Problem:** `Connection refused` to PostgreSQL

**Solution:**
1. Start PostgreSQL: `docker-compose up -d postgres`
2. Check connection: `psql postgres://postgres:postgres@localhost:5432/rustok_test`
3. Run migrations: `cd apps/server && cargo loco db migrate`

### Tests Timeout

**Problem:** Tests take too long to complete

**Solution:**
1. Reduce test data size
2. Use `#[tokio::test(flavor = "multi_thread")]` for parallel execution
3. Mock external services instead of making real calls

### Flaky Tests

**Problem:** Tests fail intermittently

**Solution:**
1. Add explicit waits for async operations
2. Use deterministic test data
3. Isolate tests (run with `--test-threads=1`)
4. Check for race conditions

### Event Not Found

**Problem:** `get_events_for_node()` returns empty vector

**Solution:**
1. Verify event is actually emitted (check logs)
2. Add wait time: `tokio::time::sleep(Duration::from_millis(100)).await;`
3. Check event subscription setup
4. Verify transaction was committed

### Auth Errors

**Problem:** `401 Unauthorized` in API calls

**Solution:**
1. Set `TEST_AUTH_TOKEN` environment variable
2. Verify auth header is sent: `app.auth_header()`
3. Check token validity in test database

### Tenant Isolation Issues

**Problem:** Tests accessing wrong tenant data

**Solution:**
1. Set `TEST_TENANT_ID` environment variable
2. Verify `X-Tenant-Id` header is sent
3. Check tenant middleware configuration
4. Use distinct tenant IDs per test

---

## Additional Resources

### Internal Documentation

- [SPRINT_4_PROGRESS.md](../../SPRINT_4_PROGRESS.md) - Sprint progress tracking
- [crates/rustok-test-utils/](../../crates/rustok-test-utils/) - Test utilities source
- [testing-guidelines.md](./testing-guidelines.md) - General testing guidelines

### External Resources

- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Tokio Testing](https://tokio.rs/tokio/topics/testing)
- [reqwest Documentation](https://docs.rs/reqwest/)
- [Wiremock](https://docs.rs/wiremock/) - HTTP mocking

---

## Changelog

### 2026-02-12
- Initial version created
- Documented test utilities framework
- Added examples for common patterns
- Added troubleshooting section

---

**Maintainer:** RusToK Team  
**Last Updated:** 2026-02-12  
**Version:** 1.0

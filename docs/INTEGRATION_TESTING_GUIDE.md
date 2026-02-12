# Integration Testing Guide

This guide explains how to write and run integration tests for the RusToK platform.

## Overview

RusToK provides a comprehensive testing infrastructure through the `rustok-test-utils` crate. This includes:

- **Test Server**: Automatically spawn a test server for HTTP-level integration tests
- **Test Fixtures**: Reusable test data builders for entities
- **Mock Event Bus**: In-memory event bus for testing event-driven flows
- **Database Utilities**: Test database setup with migrations

## Test Server

The `TestServer` type provides a complete HTTP server for integration testing.

### Basic Usage

```rust
use rustok_test_utils::TestServer;

#[tokio::test]
async fn test_http_api() {
    // Spawn a test server
    let server = TestServer::spawn().await.unwrap();

    // Make HTTP requests to the server
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health", server.base_url))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
}
```

### Using with TestApp

The `TestApp` wrapper provides convenient methods for API operations:

```rust
use rustok_test_utils::{TestServer, spawn_test_app_with_url};

#[tokio::test]
async fn test_order_flow() {
    let server = TestServer::spawn().await.unwrap();
    let app = spawn_test_app_with_url(server.base_url.clone()).await;

    // Create a product
    let product = app
        .create_product(CreateProductInput {
            sku: "TEST-001".to_string(),
            title: "Test Product".to_string(),
            price: 1000,
            ..Default::default()
        })
        .await
        .unwrap();

    // Create an order
    let order = app
        .create_order(CreateOrderInput {
            customer_id: test_customer_id(),
            items: vec![OrderItemInput {
                product_id: product.id,
                quantity: 2,
                price: Some(1000),
            }],
        })
        .await
        .unwrap();

    assert_eq!(order.status, OrderStatus::Draft);
}
```

### TestApp Methods

The `TestApp` provides the following operation methods:

#### Content/Node Operations
- `create_node(input)` - Create a content node
- `get_node(node_id)` - Retrieve a node
- `publish_node(node_id)` - Publish a node
- `add_translation(node_id, locale, input)` - Add translation
- `search_nodes(query)` - Search for nodes

#### Commerce/Product Operations
- `create_product(input)` - Create a product
- `get_product(product_id)` - Retrieve a product

#### Commerce/Order Operations
- `create_order(input)` - Create an order
- `get_order(order_id)` - Retrieve an order
- `submit_order(order_id)` - Submit an order
- `process_payment(order_id, input)` - Process payment
- `search_orders(query)` - Search for orders

#### Event Operations
- `get_events_for_node(node_id)` - Get events for a node
- `get_events_for_order(order_id)` - Get events for an order
- `get_outbox_events()` - Get all outbox events
- `get_relayed_events()` - Get count of relayed events

## Test Fixtures

The `rustok_test_utils` crate provides builder patterns for creating test data:

```rust
use rustok_test_utils::fixtures::{ProductFixture, OrderFixture};

#[tokio::test]
async fn test_with_fixtures() {
    // Create a test product
    let product = ProductFixture::new()
        .with_sku("FIX-001")
        .with_title("Fixture Product")
        .with_price(1500)
        .build();

    // Create a test order
    let order = OrderFixture::new()
        .with_customer_id(Uuid::new_v4())
        .with_item(product.id, 2, 1500)
        .build();
}
```

### Available Fixtures

- `UserFixture` - Build test users
- `TenantFixture` - Build test tenants
- `ProductFixture` - Build test products
- `OrderFixture` - Build test orders
- `NodeFixture` - Build test content nodes
- `EventFixture` - Build test domain events

## Database Testing

For tests that need database access without HTTP:

```rust
use rustok_test_utils::setup_test_db;

#[tokio::test]
async fn test_with_database() {
    let db = setup_test_db().await;

    // Use db for direct database operations
    // Note: This uses in-memory SQLite by default
}
```

### Using Migrations

For tests that need a specific schema:

```rust
use rustok_test_utils::setup_test_db_with_migrations;
use migration::Migrator;

#[tokio::test]
async fn test_with_migrations() {
    let db = setup_test_db_with_migrations::<Migrator>().await;

    // Database has all migrations applied
}
```

## Mock Event Bus

For testing event-driven behavior:

```rust
use rustok_test_utils::mock_event_bus;

#[tokio::test]
async fn test_events() {
    let event_bus = mock_event_bus();
    let tenant_id = Uuid::new_v4();

    // Publish an event
    event_bus.publish(tenant_id, None, DomainEvent::NodeCreated { .. }).unwrap();

    // Verify event was captured
    assert_eq!(event_bus.event_count(), 1);
    assert!(event_bus.has_event_of_type("NodeCreated"));
}
```

## Security Context Helpers

Create security contexts for different user roles:

```rust
use rustok_test_utils::helpers::{admin_context, customer_context};

#[tokio::test]
async fn test_permissions() {
    let admin = admin_context();
    let customer = customer_context();

    // Test with different roles
    assert!(matches!(admin.role, UserRole::Admin));
    assert!(matches!(customer.role, UserRole::Customer));
}
```

## Running Tests

### Run All Tests

```bash
cargo test --workspace
```

### Run Integration Tests Only

```bash
cargo test --package rustok-server --test '*'
```

### Run Specific Test

```bash
cargo test --package rustok-server test_complete_order_flow
```

### Run Tests with Output

```bash
RUST_LOG=debug cargo test --package rustok-server
```

## CI/CD Integration

Integration tests run automatically in CI/CD via the `integration-tests` job in `.github/workflows/ci.yml`.

The CI job:
1. Spins up a PostgreSQL database
2. Runs all integration tests
3. Runs tests sequentially to avoid port conflicts
4. Enables debug logging for troubleshooting

## Best Practices

### 1. Use TestServer for HTTP Tests

Always use `TestServer` for HTTP-level integration tests instead of expecting an external server.

### 2. Isolate Tests

Each test should be independent:
- Use unique identifiers (UUIDs)
- Clean up after each test
- Avoid shared state between tests

### 3. Use Test Fixtures

Use fixtures for consistent test data:
```rust
// Good
let product = ProductFixture::new().build();

// Avoid - inconsistent
let product = Product {
    id: Uuid::new_v4(),
    sku: "random-sku".to_string(),
    // ...
};
```

### 4. Test Both Success and Failure Cases

```rust
// Test success
let result = service.create_order(input).await;
assert!(result.is_ok());

// Test failure
let result = service.create_order(invalid_input).await;
assert!(result.is_err());
```

### 5. Verify Side Effects

```rust
// Create order
let order = create_order().await;

// Verify events were emitted
let events = get_events_for_order(order.id).await;
assert!(events.iter().any(|e| matches!(e, OrderCreated { .. })));

// Verify inventory was updated
let product = get_product(product_id).await;
assert_eq!(product.inventory, initial - order_quantity);
```

## Troubleshooting

### Port Already in Use

If you get a port conflict error:
```rust
// TestServer automatically finds an available port
// If you manually specify a port, ensure it's not in use
```

### Database Connection Errors

Ensure the test database is running:
```bash
docker-compose up -d postgres
```

### Slow Tests

If tests are slow:
- Use in-memory SQLite for unit tests
- Only use PostgreSQL when necessary
- Increase test parallelization (if tests are isolated)

## Migration from External Server Tests

If you have tests that expect an external server:

**Before:**
```rust
#[tokio::test]
#[ignore] // Requires external server
async fn test_order() {
    let app = spawn_test_app().await;
    // ...
}
```

**After:**
```rust
#[tokio::test]
async fn test_order() {
    let server = TestServer::spawn().await.unwrap();
    let app = spawn_test_app_with_url(server.base_url.clone()).await;
    // ...
}
```

## Additional Resources

- [SPRINT_4_PROGRESS.md](../SPRINT_4_PROGRESS.md) - Sprint 4 implementation details
- [SPRINT_4_START.md](../SPRINT_4_START.md) - Sprint 4 planning
- [crates/rustok-test-utils/](../crates/rustok-test-utils/) - Test utilities implementation
- [apps/server/tests/integration/](../apps/server/tests/integration/) - Example integration tests

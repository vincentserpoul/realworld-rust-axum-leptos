# Testcontainers Integration Tests

This directory contains integration tests using [Testcontainers](https://testcontainers.com/).

## What is Testcontainers?

Testcontainers is a library that provides lightweight, throwaway instances of common databases, Selenium web browsers, or anything else that can run in a Docker container. It's perfect for integration testing.

## Prerequisites

- Docker must be installed and running on your system
- Rust 1.70+ (you have 1.91.1 ✅)

## Running Tests

Run all integration tests:
```bash
cargo test --test integration_test
```

Run with output visible:
```bash
cargo test --test integration_test -- --nocapture
```

Run a specific test:
```bash
cargo test --test integration_test test_postgres_connection_with_testcontainers
```

## Available Examples

### 1. Basic PostgreSQL Connection (`test_postgres_connection_with_testcontainers`)
Shows how to:
- Start a PostgreSQL container
- Connect to it
- Run a simple query

### 2. PostgreSQL with Migrations (`test_custom_postgres_with_migrations`)
Shows how to:
- Start a PostgreSQL container
- Create tables (run migrations)
- Insert and query data

### 3. Generic Container (`test_generic_container_example`)
Shows how to:
- Use any Docker image (Redis example)
- Configure ports and wait strategies
- Get connection details

## Using Testcontainers in Your Tests

### PostgreSQL Example

```rust
use testcontainers_modules::postgres::Postgres;
use testcontainers::runners::AsyncRunner;

#[tokio::test]
async fn my_test() {
    // Start container
    let container = Postgres::default()
        .start()
        .await
        .expect("Failed to start container");

    // Get connection info
    let host = container.get_host().await.unwrap();
    let port = container.get_host_port_ipv4(5432).await.unwrap();
    
    // Use the connection info in your test...
    
    // Container is automatically cleaned up when dropped
}
```

### Other Supported Modules

The `testcontainers-modules` crate supports many databases and services:
- PostgreSQL
- MySQL
- MongoDB
- Redis
- Kafka
- ElasticSearch
- And more...

Enable additional modules in `Cargo.toml`:
```toml
testcontainers-modules = { version = "0.11.3", features = ["postgres", "redis", "mysql"] }
```

### Custom Docker Images

```rust
use testcontainers::{GenericImage, core::{ContainerPort, WaitFor}};

let image = GenericImage::new("myimage", "latest")
    .with_exposed_port(ContainerPort::Tcp(8080))
    .with_env_var("ENV_VAR", "value")
    .with_wait_for(WaitFor::message_on_stdout("Server started"));

let container = image.start().await.unwrap();
```

## Best Practices

1. **Container Lifecycle**: Containers are automatically cleaned up when the test ends
2. **Parallel Tests**: Each test gets its own container instance
3. **CI/CD**: Works great in CI - just ensure Docker is available
4. **Performance**: First run downloads images, subsequent runs are fast
5. **Isolation**: Each test is isolated with its own database

## Integration with Your Data Layer

To test your actual data layer code:

```rust
use data::clorinde::ClorindeConnectionManager;
use deadpool_postgres::{Config as PoolConfig, Runtime};

#[tokio::test]
async fn test_your_repository() {
    let container = Postgres::default().start().await.unwrap();
    let host = container.get_host().await.unwrap();
    let port = container.get_host_port_ipv4(5432).await.unwrap();

    // Configure your connection pool
    let mut config = PoolConfig::new();
    config.host = Some(host.to_string());
    config.port = Some(port);
    config.user = Some("postgres".to_string());
    config.password = Some("postgres".to_string());
    config.dbname = Some("postgres".to_string());

    let pool = config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    
    // Run your migrations here...
    
    // Test your repository methods...
}
```

## Troubleshooting

### Docker not running
```
Error: Cannot connect to Docker daemon
```
Solution: Start Docker Desktop or Docker daemon

### Port conflicts
Testcontainers automatically assigns random ports to avoid conflicts.

### Slow tests
First run downloads images. Use `docker pull postgres:11` beforehand to cache images.

## Resources

- [Testcontainers Rust Documentation](https://docs.rs/testcontainers/latest/testcontainers/)
- [Testcontainers Modules](https://docs.rs/testcontainers-modules/latest/testcontainers_modules/)
- [Official Testcontainers Website](https://testcontainers.com/)

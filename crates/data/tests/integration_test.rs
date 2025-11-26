use testcontainers::{core::{ContainerPort, WaitFor}, runners::AsyncRunner, GenericImage};
use testcontainers_modules::postgres::Postgres;
use tokio_postgres::NoTls;

#[tokio::test]
async fn test_postgres_connection_with_testcontainers() {
    // Start a PostgreSQL container
    let postgres_container = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres container");

    // Get connection details
    let host = postgres_container.get_host().await.expect("Failed to get host");
    let port = postgres_container
        .get_host_port_ipv4(5432)
        .await
        .expect("Failed to get port");

    // Create a connection string
    let connection_string = format!(
        "host={} port={} user=postgres password=postgres dbname=postgres",
        host, port
    );

    // Connect to the database
    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
        .await
        .expect("Failed to connect to database");

    // Spawn the connection in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Run a simple query
    let rows = client
        .query("SELECT version()", &[])
        .await
        .expect("Failed to execute query");

    assert!(!rows.is_empty());
    let version: String = rows[0].get(0);
    println!("PostgreSQL version: {}", version);
    assert!(version.contains("PostgreSQL"));
}

#[tokio::test]
async fn test_custom_postgres_with_migrations() {
    // Start PostgreSQL container
    let postgres_container = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres container");

    let host = postgres_container.get_host().await.expect("Failed to get host");
    let port = postgres_container
        .get_host_port_ipv4(5432)
        .await
        .expect("Failed to get port");

    let connection_string = format!(
        "host={} port={} user=postgres password=postgres dbname=postgres",
        host, port
    );

    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
        .await
        .expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Create a test table
    client
        .execute(
            "CREATE TABLE users (
                id SERIAL PRIMARY KEY,
                username VARCHAR(100) NOT NULL,
                email VARCHAR(100) NOT NULL
            )",
            &[],
        )
        .await
        .expect("Failed to create table");

    // Insert test data
    client
        .execute(
            "INSERT INTO users (username, email) VALUES ($1, $2)",
            &[&"testuser", &"test@example.com"],
        )
        .await
        .expect("Failed to insert data");

    // Query the data
    let rows = client
        .query("SELECT username, email FROM users WHERE username = $1", &[&"testuser"])
        .await
        .expect("Failed to query data");

    assert_eq!(rows.len(), 1);
    let username: String = rows[0].get(0);
    let email: String = rows[0].get(1);
    
    assert_eq!(username, "testuser");
    assert_eq!(email, "test@example.com");
}

#[tokio::test]
async fn test_generic_container_example() {
    // Example of using a generic container (Redis in this case)
    let redis_image = GenericImage::new("redis", "7-alpine")
        .with_exposed_port(ContainerPort::Tcp(6379))
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"));

    let container = redis_image
        .start()
        .await
        .expect("Failed to start Redis container");

    let host = container.get_host().await.expect("Failed to get host");
    let port = container
        .get_host_port_ipv4(6379)
        .await
        .expect("Failed to get port");

    println!("Redis is running at {}:{}", host, port);
    
    // You would typically connect to Redis here and run tests
    assert!(port > 0);
}

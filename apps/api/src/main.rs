use anyhow::Context;
use api::{routes::router, state::AppState};
use common_config::AppConfig;
use deadpool_postgres::{Config as PoolConfig, ManagerConfig, RecyclingMethod, Runtime};
use tokio::net::TcpListener;
use tokio_postgres::NoTls;
use tracing::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::load().context("failed to load configuration")?;
    let telemetry = telemetry::init_with_meter(&config.telemetry)
        .context("failed to initialize telemetry")?;

    let host = config.server.host.clone();
    let port = config.server.port;

    // Initialize database connection pool
    let mut pool_config = PoolConfig::new();
    pool_config.url = Some(config.database.url.clone());
    pool_config.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    pool_config.pool = Some(deadpool_postgres::PoolConfig::new(config.database.pool_size));
    
    let pool = pool_config
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .context("failed to create database pool")?;

    info!("database connection pool initialized");

    // Initialize repositories
    let users_repo = data::PostgresUsersRepository::new(pool.clone());
    let articles_repo = data::PostgresArticlesRepository::new(pool.clone());
    let comments_repo = data::PostgresCommentsRepository::new(pool.clone());
    
    // Initialize use cases with repositories
    let use_cases = domain::use_cases::UseCases::new(users_repo, articles_repo, comments_repo);

    // Create app state with use cases
    let state = AppState::new(use_cases);

    let app = router(state.clone(), telemetry.meter.clone());

    let listener = TcpListener::bind((host.as_str(), port))
        .await
        .with_context(|| format!("failed to bind {host}:{port}"))?;

    info!(%host, %port, "api listening");

    if let Err(err) = axum::serve(listener, app.into_make_service()).await {
        error!(?err, "server stopped unexpectedly");
        return Err(err.into());
    }

    Ok(())
}

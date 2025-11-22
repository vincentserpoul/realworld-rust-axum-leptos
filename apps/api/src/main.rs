mod auth;
mod error;
mod routes;
mod state;

use anyhow::Context;
use common_config::AppConfig;
use routes::router;
use state::AppState;
use tokio::net::TcpListener;
use tracing::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::load().context("failed to load configuration")?;
    telemetry::init(&config.telemetry).context("failed to initialize telemetry")?;

    let host = config.server.host.clone();
    let port = config.server.port;

    let state = AppState::default();
    let app = router(state.clone());

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

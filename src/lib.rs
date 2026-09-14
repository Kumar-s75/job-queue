pub mod api;
pub mod backoff;
pub mod clock;
pub mod config;
pub mod db;
pub mod error;
pub mod ids;
pub mod lease;
pub mod models;
pub mod priority;
pub mod repositories;
pub mod services;
pub mod shutdown;
pub mod state;
pub mod telemetry;
pub mod validation;

use config::Config;
use state::AppState;

pub async fn run(config: Config) -> Result<(), error::AppError> {
    telemetry::init(&config.log_filter);
    let pool = db::connect(&config.database_url, config.database_max_connections).await?;
    db::migrate(&pool).await?;
    let state = AppState::new(pool, config.clone());
    services::recovery::spawn(state.clone());
    let listener = tokio::net::TcpListener::bind(config.bind_addr).await?;
    tracing::info!(address = %config.bind_addr, "server listening");
    axum::serve(listener, api::router(state))
        .with_graceful_shutdown(shutdown::signal())
        .await?;
    Ok(())
}

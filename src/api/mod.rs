mod admin;
mod health;
mod jobs;
mod workers;

use axum::{routing::{get, post}, Router};
use tower_http::{cors::CorsLayer, request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer}, trace::TraceLayer};
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/ready", get(health::ready))
        .route("/v1/jobs", post(jobs::submit))
        .route("/v1/jobs/{id}", get(jobs::get))
        .route("/v1/jobs/{id}/heartbeat", post(jobs::heartbeat))
        .route("/v1/jobs/{id}/complete", post(jobs::complete))
        .route("/v1/jobs/{id}/fail", post(jobs::fail))
        .route("/v1/dead/{id}/retry", post(admin::retry_dead))
        .route("/v1/stats", get(admin::stats))
        .route("/v1/workers/claim", post(workers::claim))
        .with_state(state)
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}

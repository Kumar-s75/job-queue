use axum::{extract::State, Json};
use crate::{error::AppError, models::Health, state::AppState};

pub async fn health() -> Json<Health> { Json(Health { status: "ok" }) }
pub async fn ready(State(state): State<AppState>) -> Result<Json<Health>, AppError> {
    sqlx::query("SELECT 1").execute(&state.pool).await?;
    Ok(Json(Health { status: "ready" }))
}

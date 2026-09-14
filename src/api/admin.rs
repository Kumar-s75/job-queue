use axum::{extract::{Path,State},http::StatusCode,Json};
use uuid::Uuid;
use crate::{error::AppError,models::{QueueStats,RetryDeadJob},repositories::postgres,state::AppState};
pub async fn retry_dead(State(s):State<AppState>,Path(id):Path<Uuid>,Json(req):Json<RetryDeadJob>)->Result<StatusCode,AppError>{postgres::retry_dead(&s.pool,id,req.delay_seconds.unwrap_or(0)).await?;Ok(StatusCode::NO_CONTENT)}
pub async fn stats(State(s):State<AppState>)->Result<Json<Vec<QueueStats>>,AppError>{Ok(Json(postgres::stats(&s.pool).await?))}

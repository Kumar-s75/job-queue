use axum::{extract::State, Json};
use crate::{error::AppError, models::*, repositories::postgres, services::queue, state::AppState};
pub async fn claim(State(s):State<AppState>,Json(req):Json<ClaimJobs>)->Result<Json<ClaimResponse>,AppError>{queue::validate_claim(&req)?;let jobs=postgres::claim(&s.pool,req,s.config.default_lease.as_secs() as i64).await?;Ok(Json(ClaimResponse{jobs}))}

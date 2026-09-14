use axum::{extract::{Path, State}, http::StatusCode, Json};
use uuid::Uuid;
use crate::{error::AppError, models::*, repositories::postgres, services::queue, state::AppState};

pub async fn submit(State(s):State<AppState>,Json(req):Json<SubmitJob>)->Result<(StatusCode,Json<IdResponse>),AppError>{queue::validate_submit(&req)?;let id=postgres::submit(&s.pool,req).await?;Ok((StatusCode::CREATED,Json(IdResponse{id})))}
pub async fn get(State(s):State<AppState>,Path(id):Path<Uuid>)->Result<Json<Job>,AppError>{Ok(Json(postgres::get(&s.pool,id).await?))}
pub async fn heartbeat(State(s):State<AppState>,Path(id):Path<Uuid>,Json(req):Json<JobLease>)->Result<StatusCode,AppError>{postgres::heartbeat(&s.pool,id,req,s.config.default_lease.as_secs() as i64).await?;Ok(StatusCode::NO_CONTENT)}
pub async fn complete(State(s):State<AppState>,Path(id):Path<Uuid>,Json(req):Json<JobResult>)->Result<StatusCode,AppError>{postgres::complete(&s.pool,id,req).await?;Ok(StatusCode::NO_CONTENT)}
pub async fn fail(State(s):State<AppState>,Path(id):Path<Uuid>,Json(req):Json<JobResult>)->Result<StatusCode,AppError>{postgres::fail(&s.pool,id,req).await?;Ok(StatusCode::NO_CONTENT)}

use serde::Serialize;
use uuid::Uuid;
use super::Job;

#[derive(Debug, Serialize)]
pub struct IdResponse { pub id: Uuid }
#[derive(Debug, Serialize)]
pub struct ClaimResponse { pub jobs: Vec<Job> }
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct QueueStats { pub status: String, pub count: i64 }
#[derive(Debug, Serialize)]
pub struct Health { pub status: &'static str }

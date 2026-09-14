use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use super::JobStatus;

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Job {
    pub id: Uuid,
    pub queue: String,
    pub payload: Value,
    pub priority: i32,
    pub status: JobStatus,
    pub attempts: i32,
    pub max_attempts: i32,
    pub available_at: DateTime<Utc>,
    pub lease_expires_at: Option<DateTime<Utc>>,
    pub worker_id: Option<String>,
    pub claim_token: Option<Uuid>,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeadLetter { pub job: Job, pub reason: Option<String> }

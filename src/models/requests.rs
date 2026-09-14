use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SubmitJob { pub queue: String, pub payload: Value, #[serde(default)] pub priority: i32, pub max_attempts: Option<i32>, pub delay_seconds: Option<i64> }
#[derive(Debug, Deserialize)]
pub struct ClaimJobs { pub worker_id: String, pub queues: Vec<String>, pub limit: Option<i64>, pub lease_seconds: Option<i64> }
#[derive(Debug, Deserialize)]
pub struct JobLease { pub worker_id: String, pub claim_token: Uuid, pub lease_seconds: Option<i64> }
#[derive(Debug, Deserialize)]
pub struct JobResult { pub worker_id: String, pub claim_token: Uuid, pub error: Option<String> }
#[derive(Debug, Deserialize)]
pub struct RetryDeadJob { pub delay_seconds: Option<i64> }

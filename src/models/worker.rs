use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct Worker { pub id: String, pub last_heartbeat_at: DateTime<Utc>, pub started_at: DateTime<Utc> }

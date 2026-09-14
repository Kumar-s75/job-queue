use uuid::Uuid;
pub fn job_id() -> Uuid { Uuid::new_v4() }

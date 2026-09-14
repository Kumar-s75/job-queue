use crate::{error::AppError, models::*};

pub fn validate_submit(req: &SubmitJob) -> Result<(), AppError> {
    crate::validation::queue(&req.queue)?;
    if req.max_attempts.is_some_and(|n| !(1..=100).contains(&n)) { return Err(AppError::BadRequest("max_attempts must be between 1 and 100".into())); }
    if !(crate::priority::MIN..=crate::priority::MAX).contains(&req.priority) { return Err(AppError::BadRequest("priority is out of range".into())); }
    Ok(())
}

pub fn validate_claim(req: &ClaimJobs) -> Result<(), AppError> {
    crate::validation::worker(&req.worker_id)?;
    if req.queues.is_empty() { return Err(AppError::BadRequest("at least one queue is required".into())); }
    for name in &req.queues { crate::validation::queue(name)?; }
    Ok(())
}

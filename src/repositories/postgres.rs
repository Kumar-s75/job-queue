use sqlx::PgPool;
use uuid::Uuid;
use crate::{backoff, error::AppError, models::*};

pub async fn submit(pool: &PgPool, req: SubmitJob) -> Result<Uuid, AppError> {
    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO jobs(id,queue,payload,priority,max_attempts,available_at) VALUES($1,$2,$3,$4,$5,now()+($6 * interval '1 second'))")
        .bind(id).bind(req.queue).bind(req.payload).bind(req.priority).bind(req.max_attempts.unwrap_or(3)).bind(req.delay_seconds.unwrap_or(0)).execute(pool).await?;
    Ok(id)
}

pub async fn get(pool: &PgPool, id: Uuid) -> Result<Job, AppError> {
    sqlx::query_as::<_, Job>("SELECT * FROM jobs WHERE id=$1").bind(id).fetch_optional(pool).await?.ok_or(AppError::NotFound)
}

pub async fn claim(pool: &PgPool, req: ClaimJobs, default_lease: i64) -> Result<Vec<Job>, AppError> {
    let mut tx = pool.begin().await?;
    sqlx::query("INSERT INTO workers(id,last_heartbeat_at) VALUES($1,now()) ON CONFLICT(id) DO UPDATE SET last_heartbeat_at=now()")
        .bind(&req.worker_id).execute(&mut *tx).await?;
    let jobs = sqlx::query_as::<_, Job>(r#"
      WITH candidates AS (SELECT id FROM jobs WHERE status='queued' AND available_at<=now() AND queue=ANY($1) ORDER BY priority DESC, available_at, created_at FOR UPDATE SKIP LOCKED LIMIT $2)
      UPDATE jobs j SET status='running', attempts=attempts+1, worker_id=$3, claim_token=gen_random_uuid(), lease_expires_at=now()+($4 * interval '1 second'), updated_at=now()
      FROM candidates c WHERE j.id=c.id RETURNING j.*"#)
        .bind(&req.queues).bind(req.limit.unwrap_or(1).clamp(1, 100)).bind(req.worker_id).bind(req.lease_seconds.unwrap_or(default_lease).clamp(5, 3600)).fetch_all(&mut *tx).await?;
    tx.commit().await?;
    Ok(jobs)
}

async fn own(pool: &PgPool, id: Uuid, worker: &str, token: Uuid) -> Result<(), AppError> {
    let ok: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM jobs WHERE id=$1 AND status='running' AND worker_id=$2 AND claim_token=$3)").bind(id).bind(worker).bind(token).fetch_one(pool).await?;
    if ok { Ok(()) } else { Err(AppError::Conflict("job lease is not owned by this worker".into())) }
}

pub async fn heartbeat(pool: &PgPool, id: Uuid, req: JobLease, default_lease: i64) -> Result<(), AppError> {
    own(pool,id,&req.worker_id,req.claim_token).await?;
    sqlx::query("UPDATE workers SET last_heartbeat_at=now() WHERE id=$1").bind(&req.worker_id).execute(pool).await?;
    sqlx::query("UPDATE jobs SET lease_expires_at=now()+($4 * interval '1 second'),updated_at=now() WHERE id=$1 AND worker_id=$2 AND claim_token=$3")
        .bind(id).bind(req.worker_id).bind(req.claim_token).bind(req.lease_seconds.unwrap_or(default_lease).clamp(5,3600)).execute(pool).await?;
    Ok(())
}

pub async fn complete(pool: &PgPool, id: Uuid, req: JobResult) -> Result<(), AppError> {
    own(pool,id,&req.worker_id,req.claim_token).await?;
    sqlx::query("UPDATE jobs SET status='succeeded',completed_at=now(),updated_at=now(),lease_expires_at=NULL WHERE id=$1").bind(id).execute(pool).await?;
    Ok(())
}

pub async fn fail(pool: &PgPool, id: Uuid, req: JobResult) -> Result<(), AppError> {
    own(pool,id,&req.worker_id,req.claim_token).await?;
    let job = get(pool,id).await?;
    let dead = job.attempts >= job.max_attempts;
    let delay = backoff::seconds(job.attempts);
    sqlx::query("UPDATE jobs SET status=CASE WHEN $2 THEN 'dead'::job_status ELSE 'queued'::job_status END,last_error=$3,available_at=now()+($4 * interval '1 second'),worker_id=NULL,claim_token=NULL,lease_expires_at=NULL,updated_at=now(),completed_at=CASE WHEN $2 THEN now() ELSE NULL END WHERE id=$1")
        .bind(id).bind(dead).bind(req.error.unwrap_or_else(|| "worker reported failure".into())).bind(delay).execute(pool).await?;
    Ok(())
}

pub async fn recover(pool: &PgPool) -> Result<u64, AppError> {
    let result = sqlx::query("UPDATE jobs SET status=CASE WHEN attempts>=max_attempts THEN 'dead'::job_status ELSE 'queued'::job_status END,last_error='worker lease expired',available_at=now(),worker_id=NULL,claim_token=NULL,lease_expires_at=NULL,updated_at=now(),completed_at=CASE WHEN attempts>=max_attempts THEN now() ELSE NULL END WHERE status='running' AND lease_expires_at<now()")
        .execute(pool).await?;
    Ok(result.rows_affected())
}

pub async fn retry_dead(pool: &PgPool, id: Uuid, delay: i64) -> Result<(), AppError> {
    let result=sqlx::query("UPDATE jobs SET status='queued',attempts=0,last_error=NULL,completed_at=NULL,available_at=now()+($2 * interval '1 second'),updated_at=now() WHERE id=$1 AND status='dead'").bind(id).bind(delay).execute(pool).await?;
    if result.rows_affected()==0 { Err(AppError::NotFound) } else { Ok(()) }
}

pub async fn stats(pool: &PgPool) -> Result<Vec<QueueStats>, AppError> {
    Ok(sqlx::query_as("SELECT status::text AS status,count(*) AS count FROM jobs GROUP BY status ORDER BY status").fetch_all(pool).await?)
}

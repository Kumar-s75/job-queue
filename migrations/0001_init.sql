CREATE TYPE job_status AS ENUM ('queued', 'running', 'succeeded', 'dead');
CREATE TABLE jobs (
  id UUID PRIMARY KEY, queue TEXT NOT NULL, payload JSONB NOT NULL,
  priority INTEGER NOT NULL DEFAULT 0, status job_status NOT NULL DEFAULT 'queued',
  attempts INTEGER NOT NULL DEFAULT 0, max_attempts INTEGER NOT NULL DEFAULT 3,
  available_at TIMESTAMPTZ NOT NULL DEFAULT now(), lease_expires_at TIMESTAMPTZ,
  worker_id TEXT, claim_token UUID, last_error TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(), updated_at TIMESTAMPTZ NOT NULL DEFAULT now(), completed_at TIMESTAMPTZ,
  CONSTRAINT valid_attempts CHECK (max_attempts > 0 AND attempts >= 0),
  CONSTRAINT valid_queue CHECK (length(queue) BETWEEN 1 AND 128)
);
CREATE INDEX jobs_claim_idx ON jobs (queue, priority DESC, available_at, created_at) WHERE status = 'queued';
CREATE INDEX jobs_expired_idx ON jobs (lease_expires_at) WHERE status = 'running';
CREATE TABLE workers (id TEXT PRIMARY KEY, last_heartbeat_at TIMESTAMPTZ NOT NULL DEFAULT now(), started_at TIMESTAMPTZ NOT NULL DEFAULT now());

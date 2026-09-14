# ForgeQueue

A PostgreSQL-backed distributed job queue written in Rust. Multiple API instances and workers can safely coordinate using transactional `FOR UPDATE SKIP LOCKED` claims.

## Run

```sh
docker compose up --build
```

Submit and claim jobs:

```sh
curl -X POST localhost:8080/v1/jobs -H 'content-type: application/json' -d '{"queue":"emails","payload":{"to":"dev@example.com"},"priority":10}'
curl -X POST localhost:8080/v1/workers/claim -H 'content-type: application/json' -d '{"worker_id":"worker-1","queues":["emails"],"limit":1}'
```

Workers must use the returned `claim_token` to heartbeat, complete, or fail a job. Failed jobs use exponential backoff; exhausted jobs enter the dead-letter state. A recovery loop requeues abandoned leases after worker failure.

# Storage Layer Guidance

This directory defines the persistence interfaces for `aw-server-rust`. Implementations must align with the storage layer specifications under `/database/schemas`.

## PostgreSQL
- Batch insert events into the `events_raw` table using prepared statements or COPY operations.
- Partition tables by month as specified in the schema.
- Use a connection pool (e.g., `deadpool-postgres`) and apply retry logic for transient failures.
- Expose Prometheus metrics for insert throughput and error rates.

## Kafka
- Produce messages to the `events_raw` topic.
- Hash the `user_id` to choose the partition so a user's events remain ordered.
- Reuse a pooled producer and retry sends with exponential backoff.
- Monitor queue depth and publish success/failure metrics.

## MinIO
- PUT audio and screenshot objects into MinIO buckets.
- Persist only object hashes and metadata in Postgres.
- Pool connections, retry uploads, and log/metric failures.

## Transaction Coordination
- Coordinate Postgres writes, Kafka publishes, and MinIO uploads as a single logical transaction.
- Use transactional outbox or compensating actions to maintain consistency.

## Monitoring
- Integrate with tracing and metrics frameworks for each backend.
- Record latency, throughput, and error metrics.

Refer to `/database/schemas` for detailed storage specifications.

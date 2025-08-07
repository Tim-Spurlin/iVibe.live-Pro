# aw-server-rust/src Agent Instructions

This directory hosts the Rust event ingestion service that powers the platform's processing plane ingestion.

## Responsibilities
- **Batch event processing:** `ingestion/batch.rs` coordinates gRPC `EventBatch` messages and splits them into individual events.
- **Schema validation:** `ingestion/validator.rs` enforces the event schema before any storage operations.
- **TimescaleDB hypertable insertion:** `storage/postgres.rs` writes validated JSON events into hypertables for efficient time-series queries.
- **Kafka publishing:** `storage/kafka.rs` publishes each event to the `events_raw` topic for downstream consumers.
- **MinIO object storage:** `storage/minio.rs` stores binary payloads (such as audio) in object storage while keeping hashes and metadata in Postgres.
- **Debezium CDC setup:** database changes are captured and streamed to Kafka to keep processing pipelines in sync.

## Directory Layout
- `ingestion/`
- `models/`
- `storage/`
- `main.rs`

These components implement the event ingestion path defined in the schema's processing plane.

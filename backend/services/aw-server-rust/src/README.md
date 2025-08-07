# aw-server-rust Event Ingestion

This `src` directory contains the event ingestion service for `aw-server-rust`.

## Structure
- `ingestion/`
  - `batch.rs`
  - `mod.rs`
  - `validator.rs`
- `models/`
  - `event.rs`
  - `mod.rs`
- `storage/`
  - `kafka.rs`
  - `minio.rs`
  - `mod.rs`
  - `postgres.rs`
- `main.rs` *(criticality: 10)*

The service validates event batches, inserts data into TimescaleDB, publishes to the `events_raw` Kafka topic, and persists binary objects in MinIO.

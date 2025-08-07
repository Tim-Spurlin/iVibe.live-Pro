# Storage Layer

This module manages persistence for `aw-server-rust`.

## Files
- `postgres.rs` (criticality: 10) – PostgreSQL integration for `events_raw` batch inserts.
- `kafka.rs` (criticality: 9) – Kafka producer for the `events_raw` topic using `user_id` hash partitioning.
- `minio.rs` (criticality: 8) – MinIO client for audio and screenshot object storage.
- `mod.rs` (criticality: 8) – Module glue and transaction coordination.

Each backend should employ connection pooling, retry logic, and monitoring hooks. See `AGENT.md` for implementation guidance.

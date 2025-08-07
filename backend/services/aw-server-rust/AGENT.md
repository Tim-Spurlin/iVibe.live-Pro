# aw-server-rust Agent

- **Criticality:** 10/10
- **Purpose:** Event ingestion and raw storage
- **Files:** src/main.rs, src/ingestion/ (batch.rs, validator.rs), src/storage/ (postgres.rs, kafka.rs, minio.rs), src/models/
- **Receives:** EventBatch from Event Gateway
- **Writes to:** PostgreSQL (events_raw table), Kafka (events_raw topic), MinIO (audio metadata)
- **Partitioning:** By month and user_id hash
- **Throughput:** 10k events/second target


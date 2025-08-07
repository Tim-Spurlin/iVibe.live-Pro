# aw-server-rust

## Event Validation
Incoming `EventBatch` messages are validated to ensure each event conforms to the expected schema and tenant policies before any storage occurs. The validator checks timestamps, required fields, and user identifiers to prevent malformed data from entering the system.

## Storage Strategy
Validated events are inserted into the `events_raw` table in PostgreSQL. Logical replication publishes changes to the `events_raw` Kafka topic for downstream consumers. Audio object metadata is written to MinIO while the raw audio is stored separately, keeping only hashes and metadata in the database. Data is partitioned by month and a hash of `user_id` to distribute load evenly.

## TimescaleDB Hypertable Configuration
`events_raw` is configured as a TimescaleDB hypertable to support high write throughput and efficient time-based queries. A typical setup:

```sql
SELECT create_hypertable(
    'events_raw',
    'timestamp',
    partitioning_column => 'user_id_hash',
    number_partitions => 32,
    chunk_time_interval => INTERVAL '1 month'
);
```

This configuration partitions data monthly and shards by the hashed `user_id`, enabling horizontal scalability and fast lookups.

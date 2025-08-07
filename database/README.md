# Database

The iVibe platform uses PostgreSQL with the TimescaleDB and pgvector extensions. Raw events arrive in the `events_raw` hypertable and are rolled up into derived tables and materialized views for analytics and billing.

## Architecture
- **TimescaleDB** provides time‑series partitioning and compression.
- **pgvector** powers semantic search on embeddings stored in the `embeddings` table.
- Tables include `events_raw`, `embeddings`, `summaries`, `users`, and `vibe_scores`.
- Materialized views such as `mv_time_by_language`, `mv_errors_fixed`, and `mv_token_usage` support dashboards and usage tracking.

## Migration Strategy
SQL migrations live in `migrations/` and are numbered sequentially. To apply them:

1. Create a new migration file with the next numeric prefix.
2. Execute migrations using `psql` or your preferred migration runner in ascending order.
3. Avoid editing existing migrations; create follow‑up files for changes.

## Backup Procedures
- Perform nightly `pg_dump` of the entire database with compression.
- Retain at least 30 days of dumps in secure object storage.
- Test restore procedures regularly using staging environments.

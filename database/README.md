# Database

This directory contains the SQL schema and migration files for the project.

## Migration Process

Apply the migration scripts in `migrations/` sequentially using `psql` or your preferred migration tool. Each file is numbered, starting with `001_initial_schema.sql` and ending with `005_materialized_views.sql`. Run them in order to create and update the database structure. New migrations should follow the existing numbering scheme.

## Schema Design Decisions

The SQL schemas are organized into separate directories to reflect logical concerns:

- `events` — stores raw activity events as time-series data.
- `users` — authentication and profile information.
- `billing` — subscription and invoicing records.
- `emotions` — emotion detection results linked to events.
- `vibe` — aggregated vibe scores and related metrics.

This separation keeps responsibilities clear and enables selective access control per schema.

## TimescaleDB Configuration

The database runs PostgreSQL 15 with the TimescaleDB and pgvector extensions. TimescaleDB hypertables are used for time-series tables such as `events`, allowing efficient retention policies and compression. Ensure the extensions are enabled:

```sql
CREATE EXTENSION IF NOT EXISTS timescaledb;
CREATE EXTENSION IF NOT EXISTS vector;
```

Configure chunk intervals and compression policies according to the workload. pgvector powers similarity search on embeddings stored alongside events and summaries.

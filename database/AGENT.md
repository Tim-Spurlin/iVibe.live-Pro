# Database Guidelines

This directory defines the PostgreSQL schema and data management strategy for iVibe. Changes here are **criticality 9** and must maintain stability and tenant isolation.

## Tables
- **events_raw** – TimescaleDB hypertable for raw capture events.
- **embeddings** – pgvector table storing high‑dimensional embeddings.
- **summaries** – generated activity summaries with associated vectors.
- **users** – account, tier, and subscription metadata.
- **vibe_scores** – derived vibe metrics per user.

## Extensions
- [TimescaleDB](https://www.timescale.com/) for time‑series storage and partitioning.
- [pgvector](https://github.com/pgvector/pgvector) for similarity search on embeddings.

## Partitioning
- Partition **events_raw** by month and hash partitions on `user_id`.
- Ensure new partitions exist ahead of time to avoid write amplification.

## Security
- Apply row‑level security policies on all user‑facing tables to enforce tenant isolation.

## Materialized Views
- `mv_time_by_language`
- `mv_errors_fixed`
- `mv_token_usage`

## Retention
- Enforce tier‑based data retention policies across raw and aggregated data.

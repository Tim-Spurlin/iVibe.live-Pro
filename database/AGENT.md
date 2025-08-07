# Database Directory

- **Criticality:** 9/10
- **Purpose:** Database schemas and migrations
- **Subdirectories:**
  - `migrations/` – criticality: 10/10
  - `schemas/` – criticality: 9/10
  - `seeds/` – criticality: 5/10
- **Database:** PostgreSQL 15 with TimescaleDB and pgvector
- **Migration files:** `001_initial_schema.sql` through `005_materialized_views.sql`
- **Schemas:** events, users, billing, emotions, vibe

# ETL Orchestrator Source

This folder manages data aggregation and export tasks for the iVibe ETL
pipeline. Its structure is outlined below.

## Layout
- `jobs/`
  - `exports.rs`
  - `materialized_views.rs`
  - `mod.rs`
  - `retention.rs`
- `stripe/`
  - `mod.rs`
  - `usage.rs`
- `main.rs` *(criticality: 10)* – service entry point.
- `scheduler.rs` *(criticality: 9)* – cron scheduling logic.

Each component cooperates to refresh materialized views, enforce retention, and
produce CSV/JSON/PDF exports while reconciling Stripe usage.

# ETL Orchestrator

- **Criticality:** 8/10
- **Purpose:** Data aggregation and retention management
- **Files:**
  - `src/main.rs`
  - `src/scheduler.rs`
  - `src/jobs/`
    - `materialized_views.rs`
    - `retention.rs`
    - `exports.rs`
  - `src/stripe/`
    - `usage.rs`
- **Materialized views:**
  - `mv_time_by_language`
  - `mv_errors_fixed`
  - `mv_token_usage`
- **Retention:** Tier-based (7 days free, 14 days essential, unlimited pro)
- **Exports:** CSV, JSON, PDF generation
- **Stripe:** Weekly seat count reconciliation

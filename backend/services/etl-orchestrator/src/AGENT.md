# ETL Orchestrator Source Agent

- **Criticality:** 8/10
- **Owner:** Services Team

## Overview
This directory houses the runtime for the ETL orchestrator. It aggregates
activity data, enforces retention policies, and produces exports in multiple
formats. The service runs scheduled jobs that interact with PostgreSQL and other
backend services following the schema's ETL orchestration plan.

## Responsibilities
- Refresh materialized views (`mv_time_by_language`, `mv_errors_fixed`,
  `mv_token_usage`) for analytics dashboards.
- Maintain unlimited data retention for all users - no data purging.
- Generate CSV, JSON, and PDF exports for compliance and reporting.
- Communicate with PostgreSQL for all aggregation and retention operations.
- Schedule cron-style jobs via the built-in scheduler.

## Key Files
- `jobs/exports.rs` – Creates export files in supported formats.
- `jobs/materialized_views.rs` – Refreshes materialized views.
- `jobs/maintenance.rs` – Performs routine database maintenance.
- `scheduler.rs` – Defines cron triggers for ETL tasks.
- `main.rs` – Entry point that wires configuration and launches the scheduler.

## Notes
All jobs run through cron expressions and rely on PostgreSQL connections
configured in `main.rs`. Adjust schedules cautiously to avoid overlapping
execution or excessive load.

# ETL Orchestrator

The service coordinates scheduled jobs that aggregate data, enforce retention limits, and provide exports.

## ETL Schedules
- **Hourly**: refresh `mv_time_by_language`
- **Daily**: refresh `mv_errors_fixed` and `mv_token_usage`
- **Nightly**: apply tier-based retention cleanup
- **Weekly**: reconcile seat usage with Stripe and generate exports

## View Refresh Strategies
- Uses materialized views to precompute heavy queries
- Hourly views use incremental refresh; daily views run a full rebuild

## Data Retention Policies
- **Free**: 7 days of data
- **Essential**: 14 days of data
- **Pro**: unlimited retention

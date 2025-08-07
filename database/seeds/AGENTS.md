# Seed Data Instructions

This directory contains SQL seed scripts used solely for automated tests and local development.

## Demo Users
- `demo_users.sql` inserts sample accounts across subscription tiers (Free, Basic, Pro, Team, Business).
- Use anonymised details and fake emails; never commit real personally identifiable information.
- Ensure tier-specific fields align with definitions in `../schemas/users.sql`.

## Synthetic Events
- `test_events.sql` generates synthetic activity events spanning typical sources.
- Include inputs for vibe score calculations consistent with `../schemas/vibe.sql`.

## Vibe Score Calculations
- Seed data may precompute `vibe_score` or provide required inputs so database functions can derive it.
- Verify calculations against schema tests to maintain consistency.

## Cleanup Procedures
- Test runs should remove seeded data to keep environments clean:
  ```sql
  TRUNCATE TABLE events, users RESTART IDENTITY CASCADE;
  ```
  or use transaction rollbacks in the test harness.

## Data Privacy
- Demo accounts must be clearly marked and excluded from analytics.
- Never store or distribute real user data in this repository.

## Testing Requirements
- Before committing changes, ensure the schema compiles and run any database schema tests in `tests/` to confirm seeds remain valid.

# Test Data Seeds

This folder contains SQL scripts that populate the database with predictable data for tests and local development.

## Files
- **demo_users.sql** (criticality: 6) — inserts demo accounts with various activity patterns for authentication and functionality tests.
- **test_events.sql** (criticality: 6) — generates synthetic events with precomputed vibe scores for analytics validation.

## Usage
Apply these seeds only in testing environments after running all database migrations:

```bash
psql -f demo_users.sql
psql -f test_events.sql
```

Seeded records are temporary. Clean them up using the deletion queries described in `AGENT.md` once tests complete.

## Privacy
All data is synthetic and for testing purposes only. Never use real user information or production credentials.

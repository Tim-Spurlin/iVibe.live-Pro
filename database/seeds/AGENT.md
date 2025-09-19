# Seed Data Guidelines

This directory contains SQL scripts used for populating the database with deterministic data for development and automated tests.

## Demo user creation
- Provide demo accounts for testing purposes with various activity patterns.
- Use synthetic names and addresses; avoid real personally identifiable information.
- Ensure fields conform to definitions in `database/schemas` to satisfy schema testing requirements.

## Synthetic event generation
- Generate representative events for different sources (terminal, browser, mobile, audio, emotion, vibe).
- Include computed `vibe_score` values derived from stored emotion metrics.

## Vibe score calculations
- Calculate scores deterministically so tests can assert expected values.
- Persist calculated scores in `test_events.sql` for downstream analytics tests.

## Test data cleanup
- After tests, remove demo data with `DELETE` statements targeting seeded records (e.g., emails prefixed with `demo_`).
- Do not reuse seeded credentials outside automated testing environments.

## Data privacy
- All demo accounts must use fabricated information and are marked for deletion.
- Never commit real user data or sensitive tokens in these seeds.

## Testing reference
- Run database migrations before applying seeds.
- Seeds are consumed by schema tests to verify constraints, triggers, and vibe score calculations.

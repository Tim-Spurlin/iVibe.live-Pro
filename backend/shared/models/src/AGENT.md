# Shared Data Models Guide

This directory hosts data models shared across backend services.

## Event
- Defined in `event.rs`, mapping to the `events` table (`database/schemas/events.sql`).
- Struct should include `event_id: Uuid`, `user_id: Uuid`, and `timestamp: DateTime<Utc>`.
- Implements `Serialize`, `Deserialize`, and `FromRow` for database and API serialization.
- Apply validation to ensure UUID formats and required fields.

## User
- Defined in `user.rs`, mirroring the `users` table (`database/schemas/users.sql`).
- Includes tier and `subscription_id` fields to support billing.
- Derive `Serialize`, `Deserialize`, `FromRow`; validate tier enums and subscription presence.

## Vibe
- `vibe.rs` contains models for vibe scoring stored in `vibe` table (`database/schemas/vibe.sql`).
- Models capture metrics contributing to vibe scores.
- Derive `Serialize`, `Deserialize`, `FromRow` and enforce score bounds through validation.

## Common
All models must:
- Stay aligned with schema specifications in `/database/schemas`.
- Expose API contract definitions matching these structs.
- Use validation implementations to guard business rules.

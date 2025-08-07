# Models Agent Instructions

This directory hosts event data models for `aw-server-rust`.

## Event

`event.rs` defines the `Event` struct representing a single capture event stored in the database. The struct must mirror the `EventSchema` described in the repository root `AGENTS.md`.

### Fields
- `event_id` (`Uuid`)
- `user_id` (`Uuid`)
- `timestamp` (`DateTime<Utc>` RFC3339)
- `source` (`EventSource` enum with variants `Terminal`, `Browser`, `KDevelop`, `Mobile`, `Audio`, `Emotion`, `Vibe`)
- `project` (`String`)
- `language` (`Option<String>`)
- `payload` (`serde_json::Value`, stored as JSONB)

### Derives
- `serde::Serialize`
- `serde::Deserialize`
- `sqlx::FromRow`
- `Clone`, `Debug`

### Validation
- `event_id` and `user_id` must be valid UUIDs
- `timestamp` must parse as RFC3339
- `project` cannot be empty
- `source` must be one of the defined enum variants
- `payload` must be well-formed JSON


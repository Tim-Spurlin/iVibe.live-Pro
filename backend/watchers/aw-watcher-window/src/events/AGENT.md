# Event Processing Guidelines

## Files
- `mod.rs` (criticality: 8)
- `window_event.rs` (criticality: 9)

## Window Event Structure
Window events must conform to the global `EventSchema`:
- `event_id`: `uuid4`
- `user_id`: `uuid4`
- `timestamp`: RFC3339
- `source`: always `"window"`
- `project`, `language`: strings when applicable
- `payload`: window title, application identifiers, and other context

## Batching Rules
Accumulate events in memory and flush when either threshold is met:
1. **100 events** collected, or
2. **5 seconds** have elapsed since the first event in the batch.
Batches are transmitted through the gRPC client in `crate::grpc`.

### Back‑pressure
If the client cannot send fast enough, apply back‑pressure by pausing
intake until the outbound queue drains. Drop the oldest events only when
memory pressure becomes critical.

## Validation
Validate each event before enqueuing:
- all required fields present
- timestamps in chronological order
- payload fields non‑empty
Invalid events should be rejected and logged.

## Lifecycle
`WindowEvent` creation ➜ validation ➜ batch queue ➜ gRPC transmission ➜
acknowledgement from server ➜ batch cleared.

Maintain these rules whenever modifying this folder.

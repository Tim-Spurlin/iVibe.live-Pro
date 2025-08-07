# Events Module

This directory defines how window events are structured and batched before
being sent to the backend.

## Files
- `mod.rs` (criticality: 8)
- `window_event.rs` (criticality: 9)

Event data is collected, grouped into batches, and dispatched through the
watcher’s gRPC client for further processing.

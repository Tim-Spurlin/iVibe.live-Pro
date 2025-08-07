# aw-watcher-window Source

This folder contains the core window tracking implementation for the ActivityWatch window watcher.

## Structure
- `capture/`
  - `linux.rs`
  - `macos.rs`
  - `windows.rs`
  - `mod.rs`
- `events/`
  - `mod.rs`
  - `window_event.rs`
- `grpc/`
  - `client.rs`
  - `mod.rs`
- `lib.rs` – library entry point *(criticality: 9)*
- `main.rs` – binary entry point *(criticality: 10)*

The `capture` module provides platform-specific window polling, `events` defines event types, and `grpc` streams batched events to the event gateway.

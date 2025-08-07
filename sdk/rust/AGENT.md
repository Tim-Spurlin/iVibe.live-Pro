# Rust SDK Agent Guide

This directory contains the Rust SDK for third-party integrations with criticality level 7. It enables custom event capture and WebSocket subscriptions.

## Public API

- `ivibe::Client::new(api_key, endpoint)` – create a client for REST and WebSocket operations.

## Methods

- `capture_event(name, payload, tags)` – send an event to the API.
- `subscribe(channel)` – open a WebSocket subscription.
- `export_data()` – download previously captured data.

## Communication

- REST requests use HTTPS to `https://api.ivibe.live`.
- Subscriptions use WebSocket connections.

## Authentication

Include the API key in the `Authorization` header of every request.

## Event format

Events are JSON objects containing:

```json
{
  "name": "event-name",
  "payload": { /* arbitrary JSON */ },
  "tags": ["tag1", "tag2"]
}
```

## Dependencies

This crate depends on:

- `tokio` for async runtime
- `reqwest` for HTTPS requests
- `tungstenite` for WebSocket communication

Run `cargo test`, `cargo audit`, and `cargo deny` before committing changes.

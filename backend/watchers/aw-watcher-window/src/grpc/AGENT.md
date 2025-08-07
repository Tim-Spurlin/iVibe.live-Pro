# gRPC Transport Agent Instructions

This directory implements the gRPC transport layer used by `aw-watcher-window` to send events to the central event gateway.

## File Criticality
- `client.rs` – **10** – connection handling, streaming logic, retries.
- `mod.rs` – **8** – module exports and shared utilities.

## Implementation Notes
- **Transport**: Follow the event schema's transport plane protocols using gRPC over HTTP/2 on port `50051`.
- **TLS**: Validate server certificates and optionally present a client certificate. Use the system trust store or a bundled CA file.
- **Authentication**: Obtain JWTs from Keycloak via OAuth2 client credentials. Attach the token as `authorization: Bearer <token>` metadata on every call and refresh before expiration.
- **Streaming**: Stream `EventBatch` messages with back-pressure and request size checks defined by the schema.
- **Reconnection**: When the gateway connection drops, retry with exponential backoff (e.g., 1s, 2s, 4s…) up to a sane limit.
- **Local Queueing**: If the gateway remains unavailable, queue events locally and flush once a connection is re-established.

These instructions guide future development of the transport layer while keeping authentication and reliability aligned with the overall system design.

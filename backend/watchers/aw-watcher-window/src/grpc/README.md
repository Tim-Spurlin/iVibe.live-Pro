# gRPC Client for Event Gateway

This module handles all gRPC communication between `aw-watcher-window` and the event gateway.

## Features
- Streams `EventBatch` messages over port `50051` using gRPC as defined by the event schema's transport plane protocols.
- Secures traffic with TLS, validating gateway certificates and supporting client-side certificates.
- Authenticates requests by obtaining JWTs from Keycloak and attaching them as bearer tokens.
- Implements reconnection with exponential backoff and queues events locally when the gateway is unreachable.

## Files
- `client.rs` – gRPC client implementation and reconnection logic.
- `mod.rs` – Module declaration and shared exports.

See `AGENT.md` for implementation guidance and operational expectations.

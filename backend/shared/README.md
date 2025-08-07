# Shared Libraries and Utilities

This directory hosts common code used across backend services, including protobuf definitions, data models, and reusable utilities.

## Protobuf Definitions

Protobuf files live under [`proto/src/proto`](proto/src/proto):

- `events.proto`
- `gateway.proto`
- `vibe.proto`

### Regenerating Code

To regenerate Rust types from the `.proto` files, run:

```
cd backend/shared/proto
protoc --proto_path=src/proto --rust_out=src/proto src/proto/*.proto
```

Ensure `protoc` and the Rust plugin are installed before running the command.

## Data Models

The `models` crate exposes shared structs used throughout the backend:

- `Event`
- `User`
- `Vibe`

These types provide a common representation for events and user context.

## Utilities

The `utils` crate offers reusable helpers:

- `crypto` for AES-256 encryption helpers
- `telemetry` for OpenTelemetry tracing setup

```
use utils::crypto::encrypt_aes256;
use utils::telemetry::init_tracer;
```

Refer to each module for additional details and configuration options.

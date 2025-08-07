# Agent Instructions for `backend/shared/proto/src`

This directory defines the protobuf schemas and exposes the generated Rust modules.

## Protobuf and Code Generation
- Schemas are written in `proto3` and compiled with [`tonic-build`](https://docs.rs/tonic-build/latest/tonic_build/).
- After modifying `.proto` files, rerun the build script to regenerate Rust types and services.

## EventBatch Structure
- `EventBatch` aggregates multiple `Event` messages along with tenant metadata.
- Each `Event` follows the platform event schema (`event_id`, `user_id`, `timestamp`, `source`, `project`, `language`, `payload`).
- Batch ordering must be preserved; avoid changing field numbers to maintain backward compatibility.

## Streaming RPCs
- Services use gRPC streaming for efficient event ingestion (e.g., `rpc Send(stream EventBatch) returns Ack`).
- Streaming endpoints must conform to gRPC protocol specifications for flow control and error handling.

## Versioning and Field Evolution
- Use package versions such as `ivibe.v1`. For breaking changes, create a new package (`ivibe.v2`) rather than altering existing fields.
- Never reuse or renumber fields. Mark removed fields as `reserved` and append new fields with the next available number.
- Update dependent crates when versioning changes are introduced.

## Backward Compatibility
- Preserve compatibility with existing clients. Additive changes are preferred; coordinate any deprecations across services.
- Ensure generated Rust code remains stable and unit tests are updated to cover schema changes.

## Testing
- Run `cargo test` in `backend/shared/proto` after modifying schema or generated code.

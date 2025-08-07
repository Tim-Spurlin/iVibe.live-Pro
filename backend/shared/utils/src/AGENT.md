# Utilities Guidance

This directory contains shared utility modules for backend services.

## Components
- `crypto.rs` – implements AES-256-GCM authenticated encryption with keys derived using Argon2id.
- `telemetry.rs` – configures tracing and OpenTelemetry, exporting Prometheus metrics on port 9090.
- `lib.rs` – exposes utilities and centralises error handling helpers.

## Security Best Practices
- Follow repository security controls: run `cargo audit` and `cargo deny`, vendor dependencies, commit lockfiles, sign artifacts, enforce 2FA, and avoid install scripts.
- Use 256-bit keys with unique 96-bit nonces. Zeroise secrets and avoid logging sensitive data.
- Argon2id parameters should prioritise high memory cost to resist GPU attacks.

## Telemetry & Metrics
- Initialise tracing with OpenTelemetry. Propagate context across async boundaries.
- Export metrics via the Prometheus endpoint on `0.0.0.0:9090`; restrict network access to trusted hosts.

## Error Handling
- Return `Result` types and map errors to the crate's unified error enum. Do not leak internal details in log messages.

## Performance Considerations
- Reuse cipher contexts and minimise allocations during encryption/decryption.
- Keep telemetry instrumentation lightweight and bound metric label cardinality.
- Avoid blocking operations in async contexts.

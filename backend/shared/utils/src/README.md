# Utilities Module

Shared utility functions used across backend services.

## Files

| File | Description | Criticality |
|------|-------------|-------------|
| `crypto.rs` | Cryptographic helpers providing AES-256-GCM encryption with Argon2id key derivation. | 10 |
| `lib.rs` | Module exports and error handling utilities. | 8 |
| `telemetry.rs` | Tracing/OpenTelemetry setup with Prometheus metrics exporter on port 9090. | 8 |

Criticality is rated on a 1-10 scale, with 10 representing the most sensitive code.

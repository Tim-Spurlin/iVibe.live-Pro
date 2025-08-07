# Backend Workspace

This directory hosts the Rust monorepo containing all backend services for the platform. Components communicate with gRPC internally and expose REST or GraphQL interfaces externally.

## Workspace Structure

- `watchers/` – data capture clients
- `services/` – core backend services
- `intelligence/` – analytics and AI jobs
- `api/` – public and internal APIs
- `cli/` – command‑line tools
- `shared/` – common libraries and utilities

## Building

Build all crates with locked dependencies to ensure supply‑chain integrity:

```sh
cargo build --locked
```

## Testing

Run the full test suite:

```sh
cargo test
```

## Security

Audit and check dependencies regularly:

```sh
cargo audit
cargo deny check
```


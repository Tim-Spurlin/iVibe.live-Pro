# iVibe Backend

The `backend` directory contains the Rust-based core of iVibe's services. It is the heart of the platform and houses watchers, processing services, intelligence engines, APIs, the command-line interface, and shared modules.

## Workspace Structure

- `Cargo.toml` – Workspace root listing all member crates
- `Cargo.lock` – Locked dependencies for reproducible builds
- `deny.toml` – Configuration for `cargo deny`
- `watchers/` – Capture modules
- `services/` – Processing services
- `intelligence/` – AI and social engines
- `api/` – REST and GraphQL endpoints
- `cli/` – Command-line interface
- `shared/` – Common models and utilities

## Building

Build every crate in the workspace with locked dependencies:

```bash
cargo build --workspace --locked
```

## Testing

Run all tests across the workspace:

```bash
cargo test --workspace --all-targets
```

## Security Auditing

Check dependencies for supply-chain issues:

```bash
cargo deny check
cargo audit
```

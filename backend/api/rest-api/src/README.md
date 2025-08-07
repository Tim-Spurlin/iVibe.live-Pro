# REST API Source

Rust source code for the Actix Web REST API server.

## Structure

- `main.rs` – entry point (**criticality: 10**)
- `middleware/`
  - `auth.rs`
  - `mod.rs`
  - `tier.rs`
- `models/`
  - `mod.rs`
- `routes/`
  - `dashboard.rs`
  - `events.rs`
  - `export.rs`
  - `integrations.rs`
  - `mod.rs`
  - `vibe.rs`

The server listens on port 8080 and relies on JWT authentication, tier-based access control, PostgreSQL, and Redis caching. See `../README.md` for the full REST API specification.

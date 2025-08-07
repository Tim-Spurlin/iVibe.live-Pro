# Backend

- **Criticality:** 10/10
- **Purpose:** Core Rust backend services and watchers
- **Files:**
  - `Cargo.toml` (criticality: 10) - workspace configuration
  - `Cargo.lock` (criticality: 10) - locked dependencies
  - `deny.toml` (criticality: 9) - security policy
- **Subdirectories:** `watchers/`, `services/`, `intelligence/`, `api/`, `cli/`, `shared/`
- **Communication:** All backend components use gRPC internally, REST/GraphQL externally
- **Workspace members:** 19 crates total
- **Build:** `cargo build --locked` for supply chain security

# Build Scripts Agent

- **Criticality:** 8/10
- **Purpose:** Build automation for the iVibe platform.
- **Contents:**
  - `build-all.sh` – orchestrates full backend and frontend build (criticality: 8)
  - `build-frontend.sh` – builds the web assets with pnpm (criticality: 8)
  - `build-rust.sh` – compiles Rust crates (criticality: 8)

## Build Guidelines
- Follow the project's schema-driven supply chain requirements. Use `cargo audit` and `cargo deny` as specified in the root schema.
- Run Rust builds with `cargo build --locked` to ensure reproducible, vetted dependencies.
- Run frontend builds with `pnpm build --frozen-lockfile` to lock dependency versions.
- Produce Docker images for distribution. Tag images with commit hashes and sign artifacts before publishing.
- Optimise builds by leveraging parallelism (e.g. `cargo build --locked --jobs $(nproc)` and `pnpm --parallel`).
- Implement error handling: scripts must `set -euo pipefail` and emit clear logs.
- Implement rollback: on failure, remove partial artifacts, revert Docker tags, and restore previous stable builds.

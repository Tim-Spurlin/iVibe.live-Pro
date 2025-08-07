# Build Automation Scripts

This directory contains high-criticality build automation for the iVibe project.

## Scripts
- `build-all.sh` – runs both backend and frontend builds (**criticality: 8**)
- `build-frontend.sh` – builds the frontend using pnpm (**criticality: 8**)
- `build-rust.sh` – builds Rust components with cargo (**criticality: 8**)

These scripts are intended to be invoked from CI/CD or local development environments.

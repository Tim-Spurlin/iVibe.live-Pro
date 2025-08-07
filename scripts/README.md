# Automation Scripts

These scripts automate build, deployment, environment setup, and Stripe billing tasks for the iVibe.live project.

## Directory Layout
- `build/`
  - `build-all.sh`
  - `build-rust.sh`
  - `build-frontend.sh`
- `deploy/`
  - `deploy-prod.sh`
  - `rollback.sh`
- `setup/`
  - `dev-setup.sh`
  - `install-deps.sh`
  - `generate-keys.sh`
- `stripe/`
  - `create-products.sh`
  - `setup-webhooks.sh`

## Prerequisites
- Unix-like shell environment
- Rust toolchain
- Node.js
- Docker
- PostgreSQL
- Stripe CLI
- Access to Docker registry and Kubernetes cluster for deploy scripts

## Usage
1. **Setup**: run `setup/install-deps.sh` and `setup/dev-setup.sh` to prepare the environment. `setup/generate-keys.sh` creates JWT and encryption keys.
2. **Build**: run `build/build-all.sh` to compile Rust (`--locked`) and TypeScript (`--frozen-lockfile`) components.
3. **Deploy**: `deploy/deploy-prod.sh` builds and pushes Docker images, applies Kubernetes manifests, and verifies health. Use `deploy/rollback.sh` to revert if necessary.
4. **Stripe**: `stripe/create-products.sh` and `stripe/setup-webhooks.sh` configure billing via the Stripe CLI.

## Workflow
The typical automation workflow is: setup → build → deploy → stripe provisioning. These scripts are critical to the platform; review changes carefully and test in staging before production.

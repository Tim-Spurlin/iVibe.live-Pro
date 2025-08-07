# Automation Script Guidelines

This directory contains critical build, deployment, setup, and billing automation. Follow these conventions when updating scripts.

## Build Scripts
- Compile Rust projects with `--locked` to enforce `Cargo.lock`.
- Install TypeScript dependencies using a frozen lockfile (e.g., `npm ci --frozen-lockfile`).
- Fail builds on dependency drift.

## Deployment Scripts
- Build and push Docker images before rollout.
- Apply Kubernetes manifests.
- Include post-deployment health checks and rollback logic.

## Setup Scripts
- Install and configure Rust, Node.js, Docker, and PostgreSQL for development environments.

## Stripe Scripts
- Use the Stripe CLI to create products and prices.
- Configure webhook endpoints for billing events.

## Security
- Generate cryptographic keys for JWT signing and encryption.
- Never commit secrets; store them in secure vaults.

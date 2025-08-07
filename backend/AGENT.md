# Backend Agent Guide

This directory hosts the core Rust services with criticality **10**. It contains watchers, processing services, intelligence engines, APIs, the CLI, and shared modules.

## Workspace

- Manage crates through the root `Cargo.toml` workspace file and keep all member crates listed.
- Build all modules together so they share proto definitions and models.

## Dependencies and Security

- Pin dependencies via the committed `Cargo.lock` for reproducible builds.
- Enforce supply-chain security by running `cargo deny`.

## Communication

- Receives data from all capture sources.
- Sends data to PostgreSQL, Kafka, Redis, and MinIO.
- Uses gRPC internally and REST/GraphQL for external APIs.

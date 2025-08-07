# Root Orchestration Guide

**Criticality:** 10

This directory coordinates the full iVibe.live telemetry and analytics stack. It stitches together capture, transport, processing, and presentation planes so that every subsystem starts from one place.

## Architecture Overview

### Capture Plane
- `aw-watcher-window`, `aw-watcher-terminal`, `aw-watcher-browser`, and mobile watchers collect activity, emotion, and location events.
- These watchers are included in the Rust workspace defined in `Cargo.toml`.

### Transport Plane
- A gRPC event gateway relays watcher output.
- OAuth2 and JWT handling is performed here before data leaves devices.

### Processing Plane
- `aw-server-rust` ingests raw events.
- `VectoriserService` converts text and audio to embeddings.
- Additional jobs perform summarisation and ETL.

### Presentation Plane
- REST/GraphQL APIs expose processed data.
- Grafana dashboards visualise metrics for users and teams.

## Critical Files
- **Cargo.toml** – spawns all Rust modules for watchers, services, and jobs.
- **package.json** – manages TypeScript modules for the frontend and extensions.
- **docker-compose.yml** – launches PostgreSQL, Redis, Kafka, MinIO, Keycloak, and Grafana containers.
- **Makefile** – triggers `docker-compose`, `cargo`, and `pnpm` workflows.
- **.env** – supplies `DATABASE_URL`, `OPENAI_API_KEY`, `STRIPE_KEY`, and `JWT_SECRET` to every service.

## Service Communication
- PostgreSQL: `5432`
- Redis: `6379`
- Kafka: `9092`
- MinIO: `9000`
- Keycloak: `8080`
- Grafana: `3000`

## Triggers
`make` targets wire together the stack:
- `make up` starts infrastructure via `docker-compose`.
- `make build` compiles Rust crates with `cargo`.
- `make ui` installs and builds TypeScript packages with `pnpm`.

## Environment
All processes read secrets from `.env`. Ensure `DATABASE_URL`, `OPENAI_API_KEY`, `STRIPE_KEY`, and `JWT_SECRET` are set before invoking any targets.

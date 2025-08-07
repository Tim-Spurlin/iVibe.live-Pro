# Docker Services Agent

This directory contains service-specific Docker build contexts used by the Compose stacks defined in `../docker-compose*.yml`. Each subdirectory provides a Dockerfile and optional configuration files that become the service image.

## Responsibilities
- **Container definitions**: Build images for core platform services (Postgres, Keycloak, Kafka, Grafana).
- **Volume mounts**: Persist data and provisioning assets using named volumes or bind mounts.
- **Network configuration**: All services join the default `ivibe-net` bridge network as defined in the Docker Compose topology. Ports are exposed only where needed.
- **Environment variables**: Each container accepts environment variables for credentials, tuning, and optional features. Sensitive values should be injected via Compose `.env` files or secrets.
- **Health checks**: Compose defines `healthcheck` directives so dependent services wait until their upstreams report healthy status.

## Inter-service dependencies & startup order
1. **Postgres** – foundational database; must initialise before any service that needs persistent storage.
2. **Keycloak** – depends on Postgres for its database; waits for the Postgres health check.
3. **Kafka** – independent but typically available before consumers like Grafana dashboards or backend workers connect.
4. **Grafana** – relies on Postgres for the TimescaleDB backend and Keycloak for authentication.

These services are orchestrated via Docker Compose's `depends_on` and health check conditions to guarantee the order above.

## Service summaries
- **postgres/**: Base image built from `postgres:16` with `init.sql` for schema bootstrapping. Mounts a data volume at `/var/lib/postgresql/data` and exposes port `5432` within the Compose network.
- **keycloak/**: Extends the official Keycloak image. Uses `realm-export.json` for initial realm setup. Binds to Postgres via environment variables `KC_DB=postgres` and `KC_DB_URL_HOST=postgres`.
- **kafka/**: Uses an Apache Kafka image (KRaft mode). Persists logs under `/var/lib/kafka` and exposes port `9092` internally.
- **grafana/**: Custom Grafana build with provisioning files. Stores data under `/var/lib/grafana` and listens on port `3000`.

Refer to `infrastructure/docker/docker-compose.yml` for the high-level topology and to integrate additional services.

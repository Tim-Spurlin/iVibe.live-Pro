# Docker Deployment

This directory contains Docker Compose configurations for local development and production deployments of the platform.

## Architecture
The stack comprises the following containers:

- **PostgreSQL** – stores relational data.
- **Redis** – provides caching.
- **Kafka** – handles event streaming.
- **MinIO** – stores binary objects.
- **Keycloak** – manages authentication and authorisation.
- **Grafana** – visualises metrics and logs.

`docker-compose.yml` defines the baseline services, while `docker-compose.dev.yml` and `docker-compose.prod.yml` provide environment-specific overrides.

## Networking
All containers join a single bridge network, enabling name-based communication. Only the ports listed above are exposed to the host.

## Deployment
1. Copy `.env.example` to `.env` and adjust the values.
2. Start the stack in development mode:
   ```bash
   docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d
   ```
   For production:
   ```bash
   docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d
   ```
3. Named volumes preserve data for stateful services across restarts.

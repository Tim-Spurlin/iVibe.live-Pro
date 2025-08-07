# Docker Infrastructure Agent

- **Criticality:** 9/10
- **Purpose:** Docker containerization
- **Files:** `docker-compose.yml` (dev), `docker-compose.prod.yml`, `.env.example`
- **Services:** `postgres/`, `kafka/`, `keycloak/`, `grafana/`
- **Networks:** Internal bridge network for service communication
- **Volumes:** Persistent data for databases and configuration

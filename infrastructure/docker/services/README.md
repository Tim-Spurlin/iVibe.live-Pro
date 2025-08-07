# Docker Service Configurations

This directory holds build contexts for container services referenced by the Docker Compose topology.

## Contents
- **grafana/** – `Dockerfile`, `provisioning/`
- **kafka/** – `Dockerfile`
- **keycloak/** – `Dockerfile`, `realm-export.json`
- **postgres/** – `Dockerfile`, `init.sql`

Each subdirectory can be built individually or used within the Compose files in `..` to assemble the full development or production stack.

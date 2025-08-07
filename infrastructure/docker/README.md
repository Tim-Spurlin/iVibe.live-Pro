# Docker Infrastructure

This directory provides Docker Compose configurations for development and production environments.

## Container Orchestration
Use `docker-compose.yml` and environment-specific overrides (`docker-compose.dev.yml`, `docker-compose.prod.yml`) to orchestrate services like Postgres, Kafka, Keycloak, and Grafana.

## Environment Variables
Configuration values are loaded from `.env` files. Start with `.env.example`, copy it to `.env`, and adjust values to match your setup.

## Volume Management
Services use Docker volumes to persist database data and service configuration. Back up these volumes or remove them carefully when resetting the environment.

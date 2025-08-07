# Docker Orchestration Instructions

**Criticality:** 10

This directory defines Docker Compose setups for all containerized services used by the platform.

## Services
| Service     | Port | Purpose                                 |
|-------------|------|-----------------------------------------|
| PostgreSQL  | 5432 | Primary database for application data   |
| Redis       | 6379 | In-memory cache for fast lookups        |
| Kafka       | 9092 | Event streaming backbone                |
| MinIO       | 9000 | S3-compatible object storage            |
| Keycloak    | 8080 | Identity and access management          |
| Grafana     | 3000 | Metrics and dashboard visualisation     |

## Networks
- All services attach to a single bridge network for internal name resolution.
- External access is only via ports explicitly published in the compose files.

## Volumes
- Persistent data is stored in named volumes such as `postgres-data`, `redis-data`, `kafka-data`, `minio-data`, and `grafana-data`.

## Environment
- Compose files load variables from `.env`. Copy `.env.example` to `.env` and adjust values before running.

## Dependencies and Health Checks
- Use `depends_on` with `condition: service_healthy` to orchestrate startup:
  1. PostgreSQL
  2. Redis
  3. Kafka
  4. MinIO
  5. Keycloak (requires PostgreSQL)
  6. Grafana (requires PostgreSQL and Keycloak)
- Each service defines a health check (`pg_isready`, `redis-cli ping`, Kafka broker API check, `mc ls`, `curl -f http://keycloak:8080/health`, `curl -f http://grafana:3000/api/health`).

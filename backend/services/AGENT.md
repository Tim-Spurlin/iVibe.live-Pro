# Services Agent

- **Criticality:** 10/10
- **Purpose:** Core processing and ingestion services

## Subdirectories
- event-gateway (criticality: 10)
- aw-server-rust (criticality: 10)
- vectorizer-service (criticality: 9)
- summarizer-job (criticality: 8)
- etl-orchestrator (criticality: 8)
- public-profile-api (criticality: 7)
- stripe-webhook-handler (criticality: 9)

## Communication
- Kafka for event streaming
- PostgreSQL for storage
- Redis for caching

## Protocols
- gRPC internal
- REST/GraphQL external

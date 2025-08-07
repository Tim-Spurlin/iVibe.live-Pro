# Backend Services

This directory hosts the core Rust microservices responsible for event ingestion, processing, and external APIs. Internal calls use gRPC while external clients interact through REST or GraphQL endpoints. Kafka provides event streaming, PostgreSQL acts as the primary datastore, and Redis supplies caching.

## Architecture and Dependencies
1. **event-gateway** – accepts incoming batches and forwards validated events to `aw-server-rust`.
2. **aw-server-rust** – writes raw events to PostgreSQL and publishes changes to Kafka.
3. **vectorizer-service** – consumes Kafka topics to generate embeddings stored in PostgreSQL.
4. **summarizer-job** – produces daily summaries using stored events and embeddings.
5. **etl-orchestrator** – builds materialised views and exports from the persisted data.
6. **stripe-webhook-handler** – processes billing callbacks to update subscription metadata.
7. **public-profile-api** – serves cached public dashboards built from summaries and aggregates.

## Deployment Order
1. Provision PostgreSQL, Redis, and Kafka.
2. Launch **event-gateway**.
3. Start **aw-server-rust** to ingest forwarded events.
4. Run **vectorizer-service** once Kafka streams are active.
5. Enable **summarizer-job**.
6. Start **etl-orchestrator** after ingestion and summarisation are in place.
7. Deploy **stripe-webhook-handler**.
8. Expose **public-profile-api** for external access.

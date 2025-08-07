# Backend Services

This directory houses the core processing services for the platform. They collaborate to ingest events, enrich them with AI, build aggregates, and expose public APIs.

## Architecture
```
Event Gateway → aw-server-rust → Kafka → {vectorizer-service, summarizer-job} → PostgreSQL/MinIO
                                         ↘ etl-orchestrator → materialized views
                                         ↘ public-profile-api → external clients
```

### Components
- **event-gateway** – gRPC/HTTP front door for all events.
- **aw-server-rust** – validates and stores events; emits Kafka messages.
- **vectorizer-service** – transforms text and audio into embeddings.
- **summarizer-job** – generates daily summaries via OpenAI.
- **etl-orchestrator** – refreshes materialized views and applies retention.
- **public-profile-api** – serves public dashboards and JSON endpoints.
- **stripe-webhook-handler** – applies billing updates from Stripe.

## Data Flow
1. Clients send events to the **gateway**.
2. The **ingestion** service persists data in PostgreSQL with TimescaleDB and writes to Kafka.
3. **Vectorizer** and **summarizer** consume Kafka topics, calling OpenAI through the Helicone proxy.
4. Results and objects are stored in **PostgreSQL** and **MinIO**.
5. The **ETL orchestrator** aggregates data into materialized views.
6. **Public APIs** read from these views to serve users.

## Deployment
- Each service is containerized and can be orchestrated via Docker Compose or Kubernetes.
- Requires access to PostgreSQL, Kafka, MinIO, Keycloak, and Stripe credentials.
- Cron schedules trigger the summarizer and ETL jobs.
- OAuth2/JWT from Keycloak secures inter-service communication.
- Stripe webhooks must be routed to the billing handler for tier updates.

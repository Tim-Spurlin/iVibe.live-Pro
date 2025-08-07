# Services Overview

These services form the core processing pipeline of the platform and are considered **criticality level 10**.

## Event Flow
1. **Gateway** → receives all incoming events.
2. **Ingestion** → validates and writes events to storage and Kafka.
3. **Kafka** → buffers messages for downstream consumers.
4. **Vectorizer / Summarizer** → consume Kafka topics to generate embeddings and daily summaries.

## Storage
- **PostgreSQL** with TimescaleDB for structured and time-series data.
- **MinIO** for object storage such as audio payloads.

## AI Integration
- Uses **OpenAI** models accessed through the **Helicone** proxy to log usage and apply rate limits.

## Scheduling
- **Cron jobs** trigger summarization and ETL routines.

## Authentication
- All services expect OAuth2/JWT tokens issued by **Keycloak**.

## Billing
- **Stripe webhooks** update user tiers and enforce feature limits.

## Subdirectories
- `event-gateway/` – entry point for events.
- `aw-server-rust/` – event ingestion and persistence.
- `vectorizer-service/` – generates text and audio embeddings.
- `summarizer-job/` – produces daily AI summaries.
- `etl-orchestrator/` – builds materialized views and aggregates.
- `public-profile-api/` – serves public profile data.
- `stripe-webhook-handler/` – processes billing events.

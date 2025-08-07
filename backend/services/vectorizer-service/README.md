# Vectorizer Service

The vectorizer service consumes events from Kafka and turns textual or audio content into high‑dimensional embeddings.

## Embedding Models

- **Primary:** OpenAI `text-embedding-3-small` (1536 dimensions).
- **Fallback:** Local model used when external APIs are unavailable.

## Helicone Proxy Setup

All OpenAI requests pass through the Helicone proxy. Configure the following environment variables:

```bash
export HELICONE_API_BASE="https://oai.helicone.ai/v1"
export OPENAI_API_KEY="sk-..."
```

The service sends embedding requests to `text-embedding-3-small` through this proxy to capture usage metrics and provide centralized access control.

## Vector Similarity Search

Embeddings are stored in a PostgreSQL table with a `pgvector` column. Example query for nearest neighbors:

```sql
SELECT id, payload
FROM embeddings
ORDER BY embedding <-> $1
LIMIT 5;
```

This enables semantic search over previously ingested events.

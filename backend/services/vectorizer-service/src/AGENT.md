# Vectorizer Service Source Agent

- **Purpose:** Implement the vectorization stage of the schema's pipeline by turning incoming text into embeddings.
- **Kafka:** Consume the `events_raw` topic and extract textual payload fields.
- **OpenAI:** Call `text-embedding-3-small` via the Helicone proxy for remote embedding generation.
- **Fallback:** When remote calls fail, defer to local models under `local_models/`.
- **Storage:** Persist resulting vectors in PostgreSQL using the `pgvector` extension.
- **Cache:** Store embeddings in Redis with a TTL of `86400s` to avoid recalculation.

This source tree powers the embedding generation service used across the platform.

# Vectorizer Service AGENT

- **Criticality:** 9/10
- **Purpose:** Text and audio embedding generation
- **Files:** `src/main.rs`, `src/kafka_consumer.rs`, `src/openai/client.rs`, `src/openai/embeddings.rs`, `src/local_models/fallback.rs`, `src/storage.rs`
- **Consumes:** Kafka `events_raw` topic
- **API:** OpenAI `text-embedding-3-small` via Helicone proxy
- **Fallback:** Local model for offline mode
- **Stores:** PostgreSQL `pgvector` `embeddings` table
- **Dimensions:** 1536-dimensional vectors

# Vectorizer Service Source

This directory contains the embedding generation service.

## Layout
- `local_models/`
  - `fallback.rs`
  - `mod.rs`
- `openai/`
  - `client.rs`
  - `embeddings.rs`
  - `mod.rs`
- `kafka_consumer.rs` *(criticality: 10)*
- `main.rs` *(criticality: 10)*
- `storage.rs` *(criticality: 9)*

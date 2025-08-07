# Summarizer Job Source

This folder implements the AI summarization service responsible for generating daily project digests.

## Contents
- `openai/`
  - `gpt4o.rs`
  - `mod.rs`
- `main.rs` – entry point for the job (**criticality: 10**)
- `scheduler.rs` – cron scheduling (**criticality: 9**)
- `summary_builder.rs` – digest creation and embeddings (**criticality: 9**)
- `tier_check.rs` – Pro+ tier validation (**criticality: 9**)

The service retrieves the last 24 hours of events per project and, for Pro or higher tiers, sends them to the GPT‑4o model via the Helicone proxy. The resulting markdown summary and its embeddings are stored in the database.

See `AGENTS.md` for operational guidelines.

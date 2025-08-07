# Summarizer Job Source Instructions

This directory houses the AI summarization service.

## Operational Guidelines
- Run via cron daily at **04:00 UTC**.
- For each project, fetch events from the preceding **24 hours**.
- For **Pro tier and above**, invoke the OpenAI **GPT-4o** model through the **Helicone** proxy.
  - Record token usage and cost data returned by Helicone for tracking.
- Build a markdown digest summarising fetched events.
- Generate embeddings for the digest and store both summary and embedding in the `summaries` table.
- Apply tier checks: tiers below Pro skip remote GPT-4o calls.
- Refer to `/AGENTS.md` under **SummariserJob** for schema and specification details.

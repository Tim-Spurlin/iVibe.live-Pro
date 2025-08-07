# Stripe Webhook Handler Source

This directory contains the Rust code that processes Stripe billing events and syncs subscription data with PostgreSQL's billing integration.

## Structure
- **events/**
  - `invoice.rs`
  - `mod.rs`
  - `subscription.rs`
- **database.rs** – criticality: 9 – persistence layer for billing updates.
- **main.rs** – criticality: 10 – Actix Web entry point.
- **webhook.rs** – criticality: 10 – HTTP handler and signature verification.

These modules collaborate to verify Stripe callbacks, update user tiers, manage seat counts, and adjust retention policies based on the schema's billing integration.

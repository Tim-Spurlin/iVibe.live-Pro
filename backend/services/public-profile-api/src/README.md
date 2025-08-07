# Public Profile API Source

This folder implements the server-side logic for iVibe's public user profiles. Requests to `/u/{handle}` render a shareable page or return a JSON payload while respecting privacy preferences.

## Layout
- **privacy/**
  - `bitmask.rs`
  - `mod.rs`
- **templates/**
  - `profile.html`
- `cache.rs` – Redis caching utilities *(criticality: 8)*
- `handlers.rs` – `/u/{handle}` request handlers *(criticality: 9)*
- `main.rs` – service entry point *(criticality: 10)*

The service applies privacy bitmasks to user metrics, renders the `templates/profile.html` template for HTML responses, and exposes JSON endpoints for external integrations. Cached responses are stored in Redis to reduce database load.

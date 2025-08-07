# Routes Agent Instructions

This directory houses REST API route handlers. Follow these guidelines when modifying or adding endpoints.

## Endpoints

- **POST `/events`** – Accept batched event submissions. Validate the caller's tier and enforce rate limiting according to the API endpoint matrix.
- **GET `/dashboard`** – Return dashboard metrics. Responses must be cached for 60 seconds. Apply tier checks and rate limits.
- **GET `/export`** – Generate data exports in CSV, JSON, or PDF. Perform tier validation and rate limit export frequency. Reference the schema's API endpoint matrix for supported formats.
- **GET `/vibe/nearby`** – Provide real-time proximity data. Ensure tier validation and rate limiting.
- **Integration management** – Endpoints for listing, adding, and removing integrations must always enforce tier validation and rate limiting.

Consult the schema's API endpoint matrix for field definitions, tier rules, and overall route consistency.

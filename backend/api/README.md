# API Interfaces

This directory hosts the public APIs for iVibe.

## REST API

Runs on **port 8080** and exposes HTTP endpoints:

| Method | Path | Description |
| ------ | ---- | ----------- |
| `GET /events` / `POST /events` | Manage activity events |
| `GET /dashboard` | Retrieve dashboard metrics |
| `POST /integrations` | Configure third‑party integrations |
| `GET /vibe` | Access vibe scores |
| `GET /export` | Export data snapshots |

## GraphQL API

Serves on **port 8082** with a single `/graphql` endpoint that provides equivalent access to the REST resources via queries and mutations.

## Authentication

1. Obtain a JWT access token from **Keycloak** via the OpenID Connect token endpoint.
2. Include the token in requests using `Authorization: Bearer <token>`.
3. Middleware validates the signature, checks claims, applies CORS and compression, and enforces subscription tier quotas.

## Rate Limits

| Tier | Requests per minute |
| ---- | ------------------ |
| Free | 60 |
| Basic | 120 |
| Pro | 600 |
| Team | 1200 |
| Business | 6000 |

Limits apply across both REST and GraphQL endpoints. Exceeding the quota returns HTTP **429 Too Many Requests**.

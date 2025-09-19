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
3. Middleware validates the signature, checks claims, applies CORS and compression, and provides universal access to all features.

## Rate Limits

**Universal Rate Limit: 600 requests per minute for all users**

Rate limits apply for performance and security purposes only, not to restrict features. All users have access to the same generous rate limits. Exceeding the quota returns HTTP **429 Too Many Requests**.

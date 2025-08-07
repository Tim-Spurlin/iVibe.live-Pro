# Event Gateway

- **Listens on:** port 50051 (gRPC with TLS)
- **Receives from:** all watchers, mobile apps, browser extensions
- **Validates:** JWT tokens via Keycloak
- **Attaches:** `tenant_id` header for row-level security
- **Forwards to:** aw-server-rust for ingestion
- **Rate limiting:** per-user quotas
- **Dependencies:** `tonic`, `jsonwebtoken`, `keycloak_client`

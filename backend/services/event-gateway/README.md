# Event Gateway Service

The event gateway provides the first hop for all incoming event streams. It exposes a gRPC interface secured with TLS on port 50051 and applies cross‑cutting policies before handing data to downstream services.

## Gateway Pattern
- Acts as an ingress proxy for watchers, mobile apps, and browser extensions.
- Handles transport concerns such as TLS termination and back‑pressure.
- Forwards validated requests to `aw-server-rust` for ingestion.

## Authentication Flow
1. Clients obtain a JWT from Keycloak.
2. Requests are sent over TLS with the token attached.
3. The gateway verifies the token using Keycloak's public keys.
4. On success, a `tenant_id` header is attached to the request.
5. The request is forwarded to the ingestion service.

Per-user quotas are enforced via middleware to prevent abuse.

## Tenant Isolation
The `tenant_id` header allows downstream services to apply row‑level security. Each request is tagged with the tenant derived from the JWT, ensuring that data from different organisations remains isolated.

# Event Gateway

The event gateway provides a single entry point for client requests and enforces authentication, tenant isolation, and rate limits before forwarding data to downstream services.

## Authentication Flow
1. Clients obtain an access token from Keycloak using the OpenID Connect flow.
2. Incoming gRPC requests present the JWT in the `authorization` header.
3. The gateway validates the token signature and claims, rejecting expired or tampered tokens.
4. On success, the tenant ID and user information are extracted and attached to the request context.

## Rate Limits and Tenant Isolation
- Rate limiting is enforced per tenant using quotas configured in Keycloak or the gateway configuration.
- The `tenant` middleware ensures each request carries a tenant identifier used for isolation and quota tracking.
- Requests exceeding their quota receive a gRPC `RESOURCE_EXHAUSTED` status.

## Routing
Validated requests are routed to `aw-server-rust` for ingestion over gRPC on port `50051`.

## TLS Certificate Setup
Mutual TLS secures communication with internal services:
1. Generate a certificate authority (CA) used to sign service certificates.
2. Issue a server certificate for the gateway and client certificates for downstream services.
3. Configure the gateway with the server certificate and trusted CA bundle.
4. Clients must present their certificate during the TLS handshake; unmatched certificates are rejected.

This setup ensures encrypted transport and authenticated service-to-service communication.

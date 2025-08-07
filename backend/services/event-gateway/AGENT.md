# Event Gateway

- **Criticality**: 10/10
- **Purpose**: Central authentication and routing gateway
- **Owned Files**:
  - `src/main.rs`
  - `src/auth/jwt.rs`
  - `src/auth/keycloak.rs`
  - `src/grpc/server.rs`
  - `src/grpc/handlers.rs`
  - `src/middleware/rate_limit.rs`
  - `src/middleware/tenant.rs`
  - `src/tls.rs`
- **Listens on**: Port 50051 (gRPC)
- **Authentication**: Keycloak OIDC, JWT validation
- **Rate limiting**: Per-tenant quotas
- **Routes to**: aw-server-rust for ingestion
- **TLS**: Mutual TLS for internal services

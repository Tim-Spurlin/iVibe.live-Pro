# Auth Module

- **Criticality**: 10/10
- **Purpose**: Authentication and authorization for the event gateway
- **Owned Files**:
  - `jwt.rs` – criticality 10/10
  - `keycloak.rs` – criticality 10/10
  - `mod.rs` – criticality 8/10
- **Key Responsibilities**:
  - Validate JWT tokens issued by Keycloak
  - Communicate with Keycloak OIDC endpoints at `/auth/realms/ivibe`
  - Extract `tenant_id` from token claims for downstream services
  - Refresh JWKS certificates to handle Keycloak key rotation
  - Cache session data in Redis with a 3600s TTL to reduce Keycloak load
- **Reference**: See project schema for identity provider integration details

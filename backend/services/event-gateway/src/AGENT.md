# Event Gateway Source

This directory hosts the transport plane's central event routing gateway.

## Responsibilities
- gRPC server listens on **port 50051**.
- Terminates external TLS and maintains mutual TLS for internal services.
- Validates **JWTs** issued by Keycloak.
- Applies tenant isolation and rate limiting.
- Routes validated requests to downstream processing services.
- Enforces back-pressure by propagating gRPC flow control and limiting request sizes.

## Architecture
Part of the schema's **transport plane**, connecting capture clients to processing services.

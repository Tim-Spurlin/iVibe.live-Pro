# Event Gateway Source

This folder contains the central event routing gateway.

## Structure
- **auth/**
  - `jwt.rs`
  - `keycloak.rs`
  - `mod.rs`
- **grpc/**
  - `handlers.rs`
  - `mod.rs`
  - `server.rs`
- **middleware/**
  - `mod.rs`
  - `rate_limit.rs`
  - `tenant.rs`
- `main.rs` — criticality: 10
- `tls.rs` — criticality: 10

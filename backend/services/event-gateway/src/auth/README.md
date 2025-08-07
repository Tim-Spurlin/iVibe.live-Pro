# Auth Module

This directory implements authentication and authorization for the event gateway, integrating with Keycloak as the identity provider.

## Files

| File | Criticality |
| --- | --- |
| `jwt.rs` | 10 |
| `keycloak.rs` | 10 |
| `mod.rs` | 8 |

## Overview

- Validates JWT tokens and extracts `tenant_id` claims.
- Communicates with Keycloak at `/auth/realms/ivibe` for OIDC operations.
- Handles certificate rotation and caches sessions in Redis with a 3600 s TTL.
- Supports the identity provider integration defined in the project schema.

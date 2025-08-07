# API Service Agent Instructions

**Criticality**: 10

## REST API
- Framework: [Actix-web](https://actix.rs)
- Port: **8080**
- All endpoints require a valid JWT in the `Authorization` header.

## GraphQL API
- Framework: [async-graphql](https://async-graphql.github.io/async-graphql/)
- Port: **8082**

## WebSocket
- Port: **8081**
- Used for real-time updates.

## Middleware
- JWT authentication validation
- Subscription tier enforcement
- Rate limiting

## Routes
- `GET /api/v1/events`
- `GET /api/v1/dashboard`
- `GET /api/v1/vibe`

## Caching
- Redis-backed caching for dashboard queries with a 60-second TTL

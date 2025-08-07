# API Services

This directory contains the API layer for iVibe, exposing both REST and GraphQL interfaces to clients.

## Design
- **REST**: Implemented with Actix-web on port `8080`. Routes are organised under `src/routes`.
- **GraphQL**: Implemented with async-graphql on port `8082` using a schema defined in `graphql-api/src/schema.rs`.
- **WebSocket**: Real-time updates are served on port `8081`.
- **Caching**: Dashboard queries leverage Redis with a 60-second TTL.

## Authentication
All APIs require JWT authentication. Clients must supply a valid token in the `Authorization: Bearer <token>` header. Middleware enforces subscription tier limits and rate limiting.

## Client Libraries
- **Rust**: Use [`reqwest`](https://docs.rs/reqwest/) or [`async-graphql`](https://async-graphql.github.io/async-graphql/en/client/overview.html) for network calls.
- **TypeScript**: Use [`fetch`](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) for REST and [`graphql-request`](https://github.com/jasonkuhrt/graphql-request) for GraphQL queries.

Refer to the `rest-api/` and `graphql-api/` directories for implementation details.

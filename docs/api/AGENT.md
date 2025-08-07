# API Documentation Guidelines

This directory contains the canonical API specifications for the iVibe.live platform. Maintain the following conventions when updating files here:

- **REST**: Describe REST endpoints using the [OpenAPI 3.0](https://spec.openapis.org/oas/v3.0.0) specification. Update `rest/openapi.yaml` with paths, components, and examples.
- **GraphQL**: Define GraphQL types and operations using the GraphQL Schema Definition Language (SDL). Edit `graphql/schema.graphql` to reflect any changes.
- **Authentication**: Document authentication requirements for each operation. Indicate required scopes, tokens, or headers.
- **Rate Limiting**: Include rate limit details for endpoints and GraphQL operations, noting headers or policies used to convey limits.
- **Examples**: Provide example requests and responses for typical usage scenarios.
- **Versioning**: Follow semantic versioning. Introduce new versions under `/v{number}` paths or distinct schema tags.
- **Deprecation**: Mark deprecated endpoints or fields and document removal timelines. Maintain compatibility for at least one minor version.

Reference the API schemas:

- `rest/openapi.yaml`
- `graphql/schema.graphql`

Ensure documentation covers authentication, rate limits, versioning, deprecation policies, and illustrative examples.

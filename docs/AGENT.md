# Documentation Agent Instructions

This directory contains reference materials for the iVibe platform. When working on files under `/docs`, follow these guidelines.

## Architecture
- Use the four-plane model (control, data, compute, presentation) to explain the system.
- Document how components interact across planes and keep diagrams consistent with the implementation.

## API Specifications
- REST APIs are defined using **OpenAPI 3.0** in `api/rest/openapi.yaml`.
- GraphQL operations are defined in `api/graphql/schema.graphql`.
- Update these specs whenever endpoints or schema change.

## Guides
- Guides in `guides/` must provide step-by-step setup instructions with prerequisites and expected results.
- Verify each step before publishing.

## Security
- Describe the threat model, security controls, and audit procedures for the platform.
- Note authentication, authorisation, encryption, and logging requirements.

## Privacy
- Explain data handling practices and how GDPR compliance is achieved.
- Include policies on collection, storage, retention, and user consent.

## Updates
- Keep documentation in sync with code changes. Update relevant files and specs in the same commit as code modifications.

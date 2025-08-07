# Guide Authoring Instructions

This folder hosts user and developer guides for iVibe.live.

## Step-by-Step Setup
1. Ensure Docker and Node.js are installed.
2. Start local services and docs site:
   ```bash
   docker compose up -d
   npm install
   npm run docs:dev
   ```
3. Open `http://localhost:3000` to view the rendered guides.

![Development stack screenshot](../images/docker-stack.png)

## Configuration Example
Use code blocks to illustrate configuration.
```yaml
# docker-compose.yml snippet
services:
  api:
    build: ../backend
    environment:
      DATABASE_URL: postgres://ivibe:secret@localhost/ivibe
```

## Troubleshooting
- Check service status with `docker compose ps`.
- Review logs using `docker compose logs -f`.
- Verify secrets are loaded from Vault when authentication fails.

## Best Practices
- Reflect deployment requirements:
  - Docker Compose for local development.
  - Optional Kubernetes deployment via Helm charts.
  - Manage secrets with HashiCorp Vault.
  - Collect logs through Loki and metrics via Prometheus.
- Include meaningful screenshots:
  `![Grafana dashboard](../images/grafana-dashboard.png)`
- Validate all code snippets before committing.
- Follow markdown linting and keep guides up to date.

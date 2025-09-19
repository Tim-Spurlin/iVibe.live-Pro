# REST API

Actix-web powered RESTful API server for the iVibe platform.

## OpenAPI

```yaml
openapi: 3.0.0
info:
  title: iVibe REST API
  version: 1.0.0
paths:
  /api/events:
    get:
      summary: List recent events
  /api/dashboard:
    get:
      summary: Retrieve dashboard metrics (cached for 60 seconds)
  /api/vibe/nearby:
    get:
      summary: Fetch nearby vibe scores
  /api/export:
    post:
      summary: Export activity data in CSV, JSON, or PDF formats
```

## Example Requests

```bash
# List events
curl http://localhost:8000/api/events

# Retrieve dashboard (cached for 60 seconds)
curl http://localhost:8000/api/dashboard

# Vibe nearby
curl "http://localhost:8000/api/vibe/nearby?lat=51.5&lng=-0.1"

# Export data as CSV
curl -X POST "http://localhost:8000/api/export?format=csv"
```

## Universal Feature Availability

**All endpoints are available to every authenticated user without restrictions.**

| Endpoint | Description | Access Level |
|----------|-------------|---------------|
| `/api/events` | List recent events | All Users |
| `/api/dashboard` | Retrieve dashboard metrics (cached for 60 seconds) | All Users |
| `/api/vibe/nearby` | Fetch nearby vibe scores | All Users |
| `/api/export` | Export activity data in CSV, JSON, PDF formats | All Users |

All features are universally accessible. Authentication is required only for security purposes, not to restrict functionality.

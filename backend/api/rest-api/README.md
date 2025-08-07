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

## Tier-Based Feature Availability

| Endpoint | Free | Pro | Business |
|----------|:----:|:---:|:--------:|
| `/api/events` | ✅ | ✅ | ✅ |
| `/api/dashboard` | ✅ | ✅ | ✅ |
| `/api/vibe/nearby` | ✅ | ✅ | ✅ |
| `/api/export` | ❌ | CSV & JSON | CSV, JSON & PDF |

`/api/export` is available only to paid tiers. The Business tier additionally enables PDF export.

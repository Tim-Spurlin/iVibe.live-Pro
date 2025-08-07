# REST API Source Instructions

- **Server:** Actix Web running on port 8080.
- **Authentication:** JWT middleware enforces identity checks in `middleware/auth.rs`.
- **Access Control:** `middleware/tier.rs` applies tier-based permissions for each request.
- **Routes:** Handlers in `routes/` manage dashboard, events, export, integrations and vibe endpoints.
- **Validation:** Each handler validates incoming parameters and body content against the REST API schema.
- **Responses:** Serialize responses to JSON with consistent error formatting.
- **Persistence:** Query PostgreSQL for all data operations.
- **Caching:** Use Redis to cache frequently accessed resources such as dashboard metrics.
- **Specification:** Refer to the REST API schema in `../README.md` for endpoint definitions and expected models.

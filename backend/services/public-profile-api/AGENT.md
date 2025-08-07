Criticality: 7/10
Purpose: Privacy-filtered public profile serving
Files: src/main.rs, src/handlers.rs, src/privacy/ (bitmask.rs), src/cache.rs, templates/profile.html
Endpoint: /u/{handle}
Privacy: Bitmask filtering of private fields
Caching: Redis with 60-second TTL
Output: HTML via templates, JSON for API

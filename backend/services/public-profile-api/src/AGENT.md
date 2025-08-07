# Public Profile API – Source Agent

- **Scope:** `backend/services/public-profile-api/src` directory.
- **Purpose:** Serve public user profiles at `/u/{handle}` with privacy-respecting responses.

## Guidelines
- Use `handlers.rs` to route `/u/{handle}` requests.
- Apply privacy bitmasks from `privacy::bitmask` before exposing metrics.
- Render HTML via `templates/profile.html` for browser requests.
- Provide JSON responses for API clients, exposing only fields allowed by privacy settings.
- Cache profile payloads in Redis via `cache.rs` with sensible TTLs.
- Issue PostgreSQL queries that filter on privacy flags, e.g.
  ```sql
  SELECT display_name, metrics
  FROM public_profiles
  WHERE handle = $1
    AND (privacy_mask & $2) = $2;
  ```
- Reference the database schema's public profile service when adding or modifying queries.

## Privacy and Metrics
- Respect user-selected visibility; never expose metrics masked by privacy bits.
- Only share selective metrics explicitly marked as public.

## Templates
- Keep `templates/profile.html` minimal and free of sensitive data.
- Ensure all dynamic fields are escaped to prevent injection.

## Caching
- Cache HTML and JSON outputs separately.
- Invalidate cache on profile updates or privacy changes.

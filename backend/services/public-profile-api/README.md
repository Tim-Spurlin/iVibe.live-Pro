# Public Profile API

This service exposes privacy-filtered public profiles for iVibe users. The endpoint `/u/{handle}` renders the profile as HTML for browsers and provides JSON for API clients.

## Privacy Controls
Profiles are filtered using a bitmask scheme. Each field in the profile has an associated bit; when a user marks a field as private, its bit is cleared and the field is removed from the public view. Only data with the corresponding bit set is returned.

### Publicly Shareable Data
Users may choose to expose:
- basic profile information such as handle or avatar
- selected activity summaries and metrics
- any other fields explicitly marked as public

All other data remains private and is stripped before rendering.

## Caching and Invalidation
Rendered profiles are cached in Redis with a 60-second TTL to reduce load. The cache entry expires automatically after the TTL or when profile data changes, ensuring updates appear promptly.

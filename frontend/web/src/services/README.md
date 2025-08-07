# Services

This directory holds API communication services for the web frontend.

## Files
- `api.ts` – Configured Axios instance with interceptors.
- `auth.ts` – Handles JWT storage, access and refresh tokens.
- `websocket.ts` – Manages WebSocket connection to `ws://localhost:8081` with automatic reconnection.

These modules form the frontend service layer as described in the project schema, enabling secure HTTP requests and realtime updates.

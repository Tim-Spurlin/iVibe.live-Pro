# Web Frontend

## Component Hierarchy
- `src/main.tsx` bootstraps React and Redux and renders the app.
- `src/App.tsx` defines the top-level layout and React Router navigation.
- `src/components` groups UI components by domain:
  - `dashboard/`
  - `settings/`
  - `vibe/`
  - `common/`

## State Management
- Redux Toolkit manages global state.
- Slices in `src/store` include `auth.slice.ts` and `dashboard.slice.ts`.
- The configured store is provided through React Redux `<Provider>` in `src/main.tsx`.

## API Integration
- REST requests are handled through the client in `src/services/api.ts`.
- Real-time updates use a WebSocket connection from `src/services/websocket.ts`.
- Authentication helpers live in `src/services/auth.ts` and interact with the REST API.

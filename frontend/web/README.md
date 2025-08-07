# Web Frontend

This React 18 + TypeScript 5 application delivers the main dashboards and settings interface for iVibe.

## Component Structure
- `src/main.tsx` – application entry point mounting the React tree.
- `src/App.tsx` – defines routes for dashboard, vibes, settings, integrations, and profiles.
- `src/components/` – reusable UI pieces such as dashboard panels, vibe radar, and forms.
- `src/pages/` – page-level components mapped to routes.
- `src/hooks/` – custom React hooks.
- `src/services/` – REST clients targeting `http://localhost:8080/api/v1`.
- `src/store/` – Redux Toolkit store handling auth and dashboard data.
- `src/styles/` – global styles and theme configuration.
- `src/types/` – shared TypeScript definitions.
- `tests/` – placeholder for unit tests.

## State Management
Redux Toolkit provides slices for authentication and dashboard data. The store is configured under `src/store/` and supplied to components via the React provider.

## API Integration
All REST calls use the `src/services/` layer with the base URL `http://localhost:8080/api/v1`. Real-time vibe detection connects to `ws://localhost:8081`.

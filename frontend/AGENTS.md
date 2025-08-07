# Frontend Guidelines

This directory contains all TypeScript/React client applications with **criticality 10**.

## Monorepo
- Managed as a pnpm workspace; shared config lives in this folder.
- Subpackages:
  - `web/` – React web app
  - `browser-extension/` – Manifest V3 extension
  - `desktop/` – Tauri desktop app

## Communication
- Use REST and GraphQL requests to backend APIs.
- Establish a WebSocket connection to port `8081` for real-time features.

## Build
- Web and extension projects are built with **Vite**.
- Desktop app uses **Tauri** for bundling and native integration.

## Authentication
- JWT access tokens are stored in `localStorage` after login and sent in API requests.

## Real-time
- Open a WebSocket to `ws://localhost:8081` for live updates.

## Dependencies
- React
- TypeScript
- `@tanstack/react-query`
- `recharts`

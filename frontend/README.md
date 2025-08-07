# Frontend Overview

The frontend consists of three TypeScript/React applications managed in a single pnpm workspace:

- **web/** – Vite-powered web client
- **browser-extension/** – Manifest V3 extension built with Vite
- **desktop/** – Tauri desktop application

All clients communicate with backend REST and GraphQL APIs and use a WebSocket on port `8081` for live updates. Authentication uses JWTs stored in `localStorage`.

## Development Setup

1. Install prerequisites:
   - Node.js 20+
   - pnpm 8+
   - Rust (for Tauri desktop app)
2. Install workspace dependencies:
   ```bash
   pnpm install
   ```
3. Start development servers:
   - Web: `pnpm --filter web dev`
   - Browser extension: `pnpm --filter browser-extension dev`
   - Desktop: `pnpm --filter desktop tauri dev`

## Deployment

- **Web**: `pnpm --filter web build` creates a production bundle in `web/dist`.
- **Browser extension**: `pnpm --filter browser-extension build` outputs the packaged extension in `browser-extension/dist`.
- **Desktop**: `pnpm --filter desktop tauri build` generates platform-specific installers.

The applications depend on React, TypeScript, `@tanstack/react-query`, and `recharts`.

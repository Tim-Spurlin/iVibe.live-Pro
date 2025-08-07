# Web Application Guidelines

This directory contains the React web application for iVibe.live.

## Stack
- **React 18** with **TypeScript**
- **@tanstack/react-query** for data fetching and caching
- **WebSocket** connections for real-time updates
- **Redux Toolkit** for state management
- **Emotion** for styled components

## Architecture
Follow the frontend architecture defined in the repository's schema (`AGENTS.md` at the repo root). Components, hooks, services, store, and types are organised by feature to support scalability.

## Critical Files
- `App.tsx` – application shell and routing (criticality 10)
- `main.tsx` – entry point mounting the React app (criticality 10)

Adhere to TypeScript best practices and keep components focused and reusable.

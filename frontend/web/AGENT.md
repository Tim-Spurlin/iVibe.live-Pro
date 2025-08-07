# Web Frontend Agent

Criticality: **10** – primary user interface for dashboards and settings.

## Routes
- `/dashboard`
- `/vibes`
- `/settings`
- `/integrations`
- `/profile/:id`

## API Calls
- REST requests to `http://localhost:8080/api/v1`

## WebSocket
- Real-time vibe detection via `ws://localhost:8081`

## State Management
- Redux Toolkit storing authentication and dashboard data

## Core Components
- Dashboard panels
- Vibe radar
- Settings forms

## Dependencies
- React 18
- TypeScript 5
- Redux Toolkit

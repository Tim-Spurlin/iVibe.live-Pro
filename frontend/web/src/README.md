# Web Application Source

This folder houses the React web application for iVibe.live.

## Structure
- **components/**
  - **common/**
  - **dashboard/**
  - **settings/**
  - **vibe/**
- **hooks/**
  - `useAuth.ts`
  - `useEvents.ts`
  - `useVibe.ts`
- **pages/**
  - `Billing.tsx`
  - `Dashboard.tsx`
  - `Login.tsx`
  - `Profile.tsx`
  - `Settings.tsx`
- **services/**
  - `api.ts`
  - `auth.ts`
  - `websocket.ts`
- **store/**
  - `auth.slice.ts`
  - `dashboard.slice.ts`
  - `index.ts`
- **styles/**
  - `globals.css`
  - `theme.ts`
- **types/**
  - `api.ts`
  - `models.ts`
- `App.tsx` *(criticality: 10)*
- `main.tsx` *(criticality: 10)*

Refer to `AGENTS.md` in this directory for development guidelines.

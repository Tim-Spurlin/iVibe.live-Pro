# Browser Extension Source

This folder contains the browser extension implementation for iVibe. It is a Manifest V3 extension built with Vite and TypeScript.

## Directory Structure
- **background/**
  - `ai-detector.ts`
  - `index.ts`
  - `tracker.ts`
- **content/**
  - `classifier.ts`
  - `index.ts`
  - `injector.ts`
- **options/**
  - `index.html`
  - `options.tsx`
- **popup/**
  - `index.html`
  - `popup.tsx`
  - `styles.css`
- **utils/**
  - `events.ts`
  - `storage.ts`

Events recorded here are streamed via WebSocket to `ws://localhost:8081` using the platform's browser monitoring schema.

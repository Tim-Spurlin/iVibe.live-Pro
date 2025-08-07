# Browser Extension Source Guidelines

This directory contains the Manifest V3 implementation of the iVibe browser extension. It streams browser activity to the local gateway and adheres to the platform's event schema for browser monitoring.

## Architecture
- **Manifest V3** service worker lives under `background/`.
- **Background service worker** (`background/index.ts`) maintains a WebSocket to `ws://localhost:8081` for real-time streaming.
- **Content scripts** in `content/` are injected into pages to capture DOM events, URLs and AI chat interactions.
- **AI chat detection patterns** in `background/ai-detector.ts` and `content/classifier.ts` flag conversations and tag research context.
- **Research classification**: content scripts annotate browsing sessions before events are emitted.
- **Local WebSocket streaming**: events are sent to the gateway following the event schema (`event_id`, `user_id`, `timestamp`, `source='browser'`, `payload`).

## File Overview
- `background/` – service worker entry and trackers.
- `content/` – injected scripts for page analysis and classification.
- `options/` – extension configuration UI.
- `popup/` – user-facing popup interface.
- `utils/` – shared helpers for events and storage.

Use TypeScript/React conventions, build with Vite, and keep the WebSocket connection open for live updates.

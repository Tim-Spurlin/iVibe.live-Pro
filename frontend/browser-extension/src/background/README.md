# Background Service Worker

This folder implements the browser extension's background service worker. It tracks tab focus, monitors URL changes, detects AI platforms (ChatGPT, Claude, Gemini), estimates token usage, batches events, and streams them to `ws://localhost:8081`. Pending events are stored in IndexedDB until a connection is available. The worker communicates with other extension components via Chrome Extension APIs and adheres to the project's browser watcher schema.

## Files
- **index.ts** – service worker entry point. _Criticality: 10_
- **ai-detector.ts** – AI platform detection and token usage estimation. _Criticality: 9_
- **tracker.ts** – focus and URL tracker with event batching, WebSocket handling, and storage. _Criticality: 9_

# Background Service Worker Guidelines

This directory contains the background service worker for the browser extension and follows the `aw-watcher-browser` specification described in the project schema.

## Responsibilities
- Track tab focus and monitor URL changes through Chrome Extension APIs.
- Detect AI platforms such as ChatGPT, Claude, and Gemini.
- Estimate token usage for detected AI conversations.
- Batch captured events before transmission.
- Stream event batches via WebSocket to `ws://localhost:8081`.
- Store pending events in IndexedDB as a fallback when offline.
- Communicate with content scripts and other extension parts using Chrome's runtime messaging.

## File Overview
- **index.ts** (criticality 10) – service worker entry and orchestration.
- **ai-detector.ts** (criticality 9) – identifies AI platforms and estimates token usage.
- **tracker.ts** (criticality 9) – handles tab focus, URL tracking, batching, WebSocket, and storage.

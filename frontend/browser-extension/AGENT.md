# Browser Extension Agent Instructions

This directory contains the Manifest V3 browser extension responsible for high criticality (10) tracking of browsing and AI usage.

## Permissions
- `tabs`
- `webNavigation`
- `storage`
- `idle`

## Background
The service worker in `src/background/index.ts` observes tab updates and monitors the user's idle state.

## Content Scripts
`src/content/index.ts` injects helpers like `classifier.ts` to detect AI chat interfaces on visited pages.

## Classification
Machine learning routines classify activity as research or casual browsing.

## Communication
The extension streams events over a WebSocket to `ws://localhost:8081` and falls back to IndexedDB when offline.

## AI Detection
Patterns for ChatGPT, Claude, Gemini, and GitHub Copilot are recognised to log AI interaction.


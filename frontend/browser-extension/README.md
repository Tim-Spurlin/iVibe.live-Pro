# iVibe Browser Extension

This folder hosts the Manifest V3 browser extension that feeds browsing and AI usage data into the iVibe platform. The component has a criticality score of 10.

## Architecture
- **Service Worker** (`src/background/index.ts`): tracks tab changes and idle state.
- **Background Utilities** (`tracker.ts`, `ai-detector.ts`): record URLs and recognise AI assistant patterns.
- **Content Scripts** (`src/content/index.ts`, `classifier.ts`): injected into pages to classify behaviour and detect chat interfaces.
- **UI** (`src/popup/popup.tsx`, `src/options/options.tsx`): minimal popup and configuration screens.

## Privacy Features
- Events are sent to `ws://localhost:8081`; if the connection fails, data is queued in IndexedDB until it can be synced.
- Only browsing metadata needed for classification is stored.
- No third-party servers are contacted.

## Browser Compatibility
- Works on Chromium browsers that implement Manifest V3 (Chrome, Edge, Brave).
- Firefox support is experimental and may require additional polyfills.


# Browser Extension

This extension tracks browser activity for iVibe.live.

## Installation
1. Clone the repository and navigate to `frontend/browser-extension`.
2. In Chrome, Edge, or Brave:
   - Open `chrome://extensions`.
   - Enable **Developer mode**.
   - Choose **Load unpacked** and select this folder.
3. In Firefox:
   - Open `about:debugging#/runtime/this-firefox`.
   - Click **Load Temporary Add-on** and select `manifest.v3.json`.
4. Safari uses the **Develop > Show Web Extension** menu from Safari 14 and later.

## Permissions
- **Tabs**: read the active tab URL and track focus time.
- **Scripting**: inject content scripts to measure scroll depth and detect AI chats.
- **Storage**: retain local state between sessions.

## Privacy
Captured data is sent only to a local WebSocket at `ws://localhost:8081` and is never transmitted to third-party servers. The extension classifies visits as research or casual browsing and records URLs, tab focus, AI chat usage, and scroll depth for personal analytics.

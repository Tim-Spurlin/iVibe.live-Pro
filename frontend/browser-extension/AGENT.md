# AGENT Instructions

- **Criticality**: 10/10
- **Purpose**: Browser activity tracking extension
- **Files**: manifest.v3.json (Manifest V3), src/background/ (service worker), src/content/ (content scripts), src/popup/ (extension popup)
- **Tracks**: URLs, tab focus, AI chat detection, scroll depth
- **Classification**: Research vs casual browsing
- **Communication**: WebSocket to localhost:8081
- **Supported browsers**: Chrome, Firefox, Edge, Brave, Safari

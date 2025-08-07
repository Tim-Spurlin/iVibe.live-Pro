# Watchers

- **Criticality:** 10/10
- **Purpose:** Data capture modules for all platforms
- **Subdirectories:**
  - `aw-watcher-window` (criticality: 10)
  - `aw-watcher-kdevelop` (criticality: 10)
  - `aw-watcher-terminal` (criticality: 10)
  - `aw-watcher-audio` (criticality: 8)
- **Communication:** Each watcher → Event Gateway via gRPC over TLS
- **Event batch size:** 100 events or 5 seconds
- **Protocols:** gRPC with protobuf serialization
- **Platform-specific:** Linux (DBus, X11), Windows (Win32 API), macOS (Accessibility API)

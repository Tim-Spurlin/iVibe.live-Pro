# aw-watcher-window Agent

- **Criticality:** 10/10
- **Purpose:** Window focus and application tracking across platforms
- **Files:** `Cargo.toml`, `src/main.rs` (entry point), `src/lib.rs` (library), `src/capture/` (platform modules), `src/events/` (event types), `src/grpc/` (client)
- **Platform modules:**
  - `linux.rs` (X11/Wayland)
  - `windows.rs` (Win32 API)
  - `macos.rs` (Accessibility API)
- **Captures:** Window title, application name, file path, project context
- **Update frequency:** 1–5 seconds
- **Output:** `WindowEvent` → `EventBatch` → gRPC stream


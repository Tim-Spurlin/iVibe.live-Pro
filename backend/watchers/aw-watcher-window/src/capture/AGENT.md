# Platform Capture Guidelines

This directory hosts per-platform modules that feed window activity into iVibe's event pipeline. Each implementation listens for native window focus changes, extracts metadata, and forwards structured events to `aw-server-rust` via gRPC `EventBatch` messages.

## linux.rs (criticality: 10)
- **Hooks**: Listens to focus events from X11 or Wayland compositors; queries DBus when session managers expose additional hints.
- **Extraction**: Reads window titles, application names, and file paths via `_NET_WM_NAME` or DBus properties.
- **Fallbacks**: Polls the root window and DBus when event streams fail.
- **Compatibility**: Normalises output into the shared `WindowEvent` schema before batching.

## macos.rs (criticality: 10)
- **Hooks**: Subscribes to Accessibility API callbacks and `NSWorkspace` notifications for the frontmost application.
- **Extraction**: Retrieves window title, bundle identifier, and document URL through `AXTitle` and `AXDocument` attributes.
- **Fallbacks**: Falls back to periodic polling when Accessibility notifications are denied.
- **Compatibility**: Maps data into the cross-platform schema and streams via gRPC.

## windows.rs (criticality: 10)
- **Hooks**: Uses `SetWinEventHook` and UIAutomation to detect foreground window changes.
- **Extraction**: Captures window text (`GetWindowTextW`), process names, and document paths via Automation patterns.
- **Fallbacks**: Polls `GetForegroundWindow` when hooks are unavailable.
- **Compatibility**: Ensures UTF-8 normalization and emits schema-compliant events.

## mod.rs (criticality: 9)
- Selects the appropriate platform implementation using `cfg` attributes.
- Exposes a common trait to start/stop capture and emit `WindowEvent` instances.
- Maintains uniform communication patterns across platforms.

All modules must conform to the global `EventSchema` (`event_id`, `user_id`, `timestamp`, `source`, `payload`), ensuring consistent cross-platform behaviour.

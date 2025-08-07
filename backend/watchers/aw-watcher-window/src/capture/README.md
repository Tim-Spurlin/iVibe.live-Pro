# Window Capture Modules

This directory contains platform-specific implementations for capturing active window information in `aw-watcher-window`.

| File | Criticality | Purpose |
| --- | --- | --- |
| `linux.rs` | 10 | Captures window events on Linux using X11/Wayland and DBus. |
| `macos.rs` | 10 | Uses the macOS Accessibility API and `NSWorkspace` notifications. |
| `windows.rs` | 10 | Hooks into Win32 events and UIAutomation on Windows. |
| `mod.rs` | 9 | Selects the correct platform module and exposes a common interface. |

Each module reports window titles, application names, and file paths to the shared `aw-server-rust` backend.

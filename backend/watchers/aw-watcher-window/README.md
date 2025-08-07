# aw-watcher-window

Cross-platform desktop window focus tracker.

## Platform Capture Methods
- **Linux:** Hooks into `_NET_ACTIVE_WINDOW` via X11/DBus. If X11 is absent, the watcher reads the `/proc` filesystem for active window metadata.
- **Windows:** Uses the Win32 `EVENT_SYSTEM_FOREGROUND` event to detect focus changes.
- **macOS:** Leverages `NSWorkspace` through the Accessibility API to monitor the frontmost application.

## Event Batching
Captured events contain the window title, application name, and file path. Events are batched and sent to the Event Gateway over gRPC when either 100 events are collected or 5 seconds have passed, whichever comes first.

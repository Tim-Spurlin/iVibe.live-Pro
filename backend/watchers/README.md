# Watchers

Platform-specific collectors that capture user activity and stream events to the Event Gateway. Each watcher buffers up to 100 events or 5 seconds before sending a batch over gRPC with protobuf serialization and TLS.

## Modules

### aw-watcher-window
Captures active window titles and focus events across desktop environments using DBus and X11 on Linux, the Win32 API on Windows, and the Accessibility API on macOS.

### aw-watcher-kdevelop
Monitors the KDevelop IDE, recording opened files, project context, and branch information through DBus signals.

### aw-watcher-terminal
Observes shell sessions, logging executed commands, exit codes, and error output for supported shells.

### aw-watcher-audio
Streams microphone activity and optional hotword detections while applying privacy filters before vectorization.

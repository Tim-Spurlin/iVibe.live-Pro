# aw-watcher-kdevelop/src

This directory hosts the source code for the KDevelop IDE watcher. It implements the KTextEditor plug-in that captures editor activity and forwards structured events to the ActivityWatch event gateway.

## Files

| File      | Criticality | Description |
|-----------|-------------|-------------|
| `main.rs` | 10/10 | Entry point that initialises DBus listeners and launches the watcher. |
| `plugin.rs` | 10/10 | Registers the watcher as a KDevelop/KTextEditor plug-in and hooks editor callbacks. |
| `dbus.rs` | 9/10 | Handles DBus communication with KDevelop to receive KTextEditor signals. |
| `events.rs` | 8/10 | Defines event structures and helper functions for sending records to the gateway. |

These modules collectively provide IDE integration, monitoring file open/save, compile, and debug events while attaching project, branch, and language metadata as required by the platform's EventSchema.

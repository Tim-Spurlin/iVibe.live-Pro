# aw-watcher-kdevelop

A watcher that hooks into KDevelop to record editor activity for ActivityWatch. It registers as a KTextEditor plugin and sends structured events through DBus to the ActivityWatch event gateway.

## Installation

1. Build the watcher:
   ```bash
   cargo build --release
   ```
2. Copy the produced plugin library from `target/release` into your KDevelop plugin directory (usually `~/.local/share/kdevelop/plugins`).
3. Start KDevelop and enable the **ActivityWatch KDevelop** plugin.
4. Run the watcher binary to forward events to your ActivityWatch backend.

## Captured Metrics

- File open and save operations
- Project and branch switches
- Compilation results
- Debug session start and stop
- Detected programming language and file path

These metrics provide detailed insight into developer workflow inside KDevelop and are forwarded to the ActivityWatch event gateway for aggregation and analysis.

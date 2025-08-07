# AGENT Instructions

This directory contains the cross-platform window watcher for the capture plane.

## Platform Communication
- **Linux:** capture active window events via DBus.
- **Windows:** use Win32 APIs to monitor foreground window changes.
- **macOS:** rely on the Accessibility API for window focus notifications.

Keep platform-specific logic isolated within the `capture/` module.

## Event Flow
- Batch window events every 1–5 seconds to minimise overhead.
- Stream batches through the gRPC client to the event gateway.
- Ensure emitted events follow the capture plane schema (event_id, user_id, timestamp, source, project, language, payload).

## Architecture Notes
This watcher operates in the capture plane and should only handle collection and forwarding of window events. Processing or storage belongs elsewhere in the pipeline.

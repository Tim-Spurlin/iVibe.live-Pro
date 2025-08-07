# aw-watcher-window – Agent Instructions

## Capture Details
- Record `window_title`, `app_name`, and `file_path` every 1-5 seconds.
- Batch events to the Event Gateway via gRPC, sending when batch reaches 100 events or every 5 seconds.

## Platform Hooks
- **Linux:** Uses `_NET_ACTIVE_WINDOW` through X11/DBus. Fall back to the proc filesystem if X11 is unavailable.
- **Windows:** Relies on `EVENT_SYSTEM_FOREGROUND` via the Win32 API.
- **macOS:** Uses `NSWorkspace` from the Accessibility API.

## Dependencies
- `x11`
- `winapi`
- `cocoa`
- `tokio`
- `tonic`

## Testing
- `cargo test --locked`
- Attempt to run `cargo audit` and `cargo deny`

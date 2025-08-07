# aw-watcher-window

Cross-platform window focus and application tracker used by iVibe.live. The watcher captures details about the currently active window and streams them to `aw-server-rust` over gRPC.

## Building

### Linux
- Dependencies: `libx11-dev`, `pkg-config`, and a running X11 or Wayland session.
- Build:
  ```sh
  cargo build --bin aw-watcher-window
  ```

### Windows
- Install the MSVC toolchain via [Rustup](https://rustup.rs/).
- Build:
  ```powershell
  cargo build --bin aw-watcher-window --target x86_64-pc-windows-msvc
  ```

### macOS
- Requires Xcode command-line tools.
- Grant Terminal (or the watcher binary) [Accessibility](https://support.apple.com/guide/mac-help/app-permissions-mchl77879b8a/mac) permissions: *System Settings → Privacy & Security → Accessibility*.
- Build:
  ```sh
  cargo build --bin aw-watcher-window --target x86_64-apple-darwin
  ```
  For Apple Silicon use `aarch64-apple-darwin`.

## Permissions
- **Linux:** ability to connect to the X11 display or Wayland compositor.
- **Windows:** no special permissions required beyond running under the current user.
- **macOS:** Accessibility permission is mandatory to read window information.

## Event schema
Each event represents a snapshot of the focused window:

```json
{
  "timestamp": "2024-06-01T12:34:56Z",
  "title": "main.rs — Visual Studio Code",
  "application": "code",
  "path": "/home/alice/projects/ivibe/src/main.rs",
  "project": "ivibe"
}
```

Events are produced every 1–5 seconds, batched into an `EventBatch`, and sent to the server via gRPC.

## Example
A batch containing two events:

```json
{
  "hostname": "workstation",
  "user_id": "a1b2c3d4",
  "events": [
    {
      "timestamp": "2024-06-01T12:34:56Z",
      "title": "main.rs — Visual Studio Code",
      "application": "code",
      "path": "/home/alice/projects/ivibe/src/main.rs",
      "project": "ivibe"
    },
    {
      "timestamp": "2024-06-01T12:35:02Z",
      "title": "README.md — vim",
      "application": "vim",
      "path": "/home/alice/projects/ivibe/README.md",
      "project": "ivibe"
    }
  ]
}
```


# Tauri Backend Agent

This directory contains the Rust backend for the iVibe desktop client built with Tauri. As defined in the project's cross-platform client specification, the desktop app combines a Rust core with a TypeScript UI wrapped by Tauri and integrates activity watchers and the GraphQL API.

## Responsibilities
- **Window management**: create and manage application windows with `tauri::Window`, persist size and position, and handle multi-window scenarios.
- **System tray integration**: register tray icon and menus, respond to events to show, hide, or quit the app.
- **IPC bridge**: expose Rust commands via `invoke_handler` and emit events to the frontend for bidirectional communication.
- **Native API access**: surface file system, notification, and OS-level features through Tauri commands.
- **Auto-updater**: configure the Tauri updater in `tauri.conf.json`, including update endpoints and signature verification.
- **Code signing**: enforce signing for release builds—Developer ID and notarisation on macOS, SignTool with timestamp on Windows, and optional GPG signatures on Linux.

## Platform-specific build configurations
- **Linux**: target AppImage and `.deb` bundles, specify icons in `icons/`.
- **macOS**: set bundle identifier and signing identity; produce a signed `.app` and `.dmg`.
- **Windows**: configure MSI installer options and code-signing certificate path.

## Critical files
- `Cargo.toml` – Rust crate manifest (criticality 9).
- `tauri.conf.json` – Tauri configuration (criticality 10).


# Desktop Application

This directory contains the Tauri-based desktop client. The app combines a React frontend with a Rust backend to deliver native performance and a small footprint.

## Prerequisites

- Node.js and [pnpm](https://pnpm.io/) installed
- Rust toolchain with `cargo`
- Tauri CLI (`pnpm add -g @tauri-apps/cli` or `cargo install tauri-cli`)
- Platform build tools (Xcode on macOS, Visual Studio Build Tools on Windows, required GTK libraries on Linux)

## Building

Run all commands from this folder.

### Windows (MSI)
```bash
pnpm tauri build --target x86_64-pc-windows-msvc
```

**Signing:** provide a `.pfx` code-signing certificate and expose it to Tauri via:
```powershell
set TAURI_WINDOWS_CERTIFICATE=path\to\cert.pfx
set TAURI_WINDOWS_PASSWORD=yourPassword
```

### macOS (DMG)
```bash
pnpm tauri build --target aarch64-apple-darwin
```

**Signing:** requires an Apple Developer ID Application certificate:
```bash
export APPLE_CERTIFICATE="/path/to/cert.p12"
export APPLE_CERTIFICATE_PASSWORD="password"
export APPLE_TEAM_ID="YOURTEAMID"
export APPLE_SIGNING_IDENTITY="Developer ID Application: Your Name (TEAMID)"
```

### Linux (DEB/RPM)
```bash
pnpm tauri build --target x86_64-unknown-linux-gnu
```

**Signing:** sign packages with GPG:
```bash
gpg --detach-sign --armor path/to/app_*.deb
gpg --detach-sign --armor path/to/app-*.rpm
```

## Development

To run in development mode with hot reload:
```bash
pnpm tauri dev
```

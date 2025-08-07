# Tauri Desktop Backend

This directory hosts the backend for the iVibe desktop application built with Tauri.

## Structure
- `icons/` – application icons placeholder (`.gitkeep`).
- `src/` – Rust source code (`main.rs`).
- `Cargo.toml` – crate manifest (criticality 9).
- `tauri.conf.json` – Tauri configuration (criticality 10).

The frontend UI for the desktop app lives alongside this folder and communicates with this backend through Tauri's IPC bridge.

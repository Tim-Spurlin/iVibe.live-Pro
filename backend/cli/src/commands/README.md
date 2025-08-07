# CLI Command Implementations

This folder contains the implementation of individual commands for the `ivb` CLI. These modules follow the project's CLI tool specification for authentication and output formatting.

## Files

- `capture.rs` (criticality: 9) – submit manual events to the API.
- `export.rs` (criticality: 8) – retrieve stored data as JSON or CSV.
- `account.rs` (criticality: 8) – manage login and subscription state.
- `config.rs` (criticality: 8) – read and update local settings.
- `mod.rs` (criticality: 8) – wire commands into the CLI dispatcher.

All commands require API authentication and provide robust error handling.

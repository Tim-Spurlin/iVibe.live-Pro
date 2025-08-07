# Agent Instructions

- This directory houses the `ivb` CLI tool implementation.
- Command parsing is handled by [`clap`](https://crates.io/crates/clap); extend commands within the `commands` module.
- Configure the API client in `client.rs` to communicate with iVibe HTTPS endpoints.
- Support event capture for headless sessions through capture-related commands.
- Implement data export features for offline analysis.
- Provide account management operations such as login and profile handling.
- Read and write configuration values from `~/.ivibe/config.toml` via the `config` command.
- Ensure all interactions respect the iVibe schema's CLI capabilities.

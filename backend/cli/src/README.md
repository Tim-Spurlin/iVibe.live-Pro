# ivb CLI Source

This folder contains the implementation of the `ivb` command-line tool for the iVibe platform.

## Structure

- `commands/`
  - `account.rs`
  - `capture.rs`
  - `config.rs`
  - `export.rs`
  - `mod.rs`
- `client.rs` _(criticality: 9)_
- `main.rs` _(criticality: 10)_

Configuration is read from `~/.ivibe/config.toml`, and all API communication uses HTTPS endpoints.

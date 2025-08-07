# iVibe CLI

`ivb` is the command-line interface for interacting with the iVibe platform.

## Installation

Install from the repository by building with Cargo:

```bash
cargo install --path backend/cli
```

## Usage

Examples of available commands:

```bash
# capture activity from a given source
ivb capture --source terminal

# export collected data to a CSV file
ivb export --format csv --output activity.csv

# manage account login and profile
ivb account login

# update local configuration options
ivb config set api_url https://api.ivibe.live
```

## Configuration

`ivb` reads configuration from a TOML file, typically located at `~/.config/ivb/config.toml`:

```toml
[server]
url = "https://api.ivibe.live"

[auth]
token = "your-api-token"
```

This file defines the API endpoint and authentication token used by the CLI.

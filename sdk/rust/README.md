# iVibe Rust SDK

Rust SDK for integrating third-party applications with the iVibe platform. The library allows you to capture custom events, subscribe to real-time updates, and export your data.

## Installation

Add this crate to your `Cargo.toml`:

```toml
ivibe = { git = "https://github.com/ivibe/ivibe-sdk", package = "ivibe-sdk" }
# or once published on crates.io:
# ivibe = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Authentication

Every request must include your API key:

```rust
let client = ivibe::Client::new("YOUR_API_KEY", "https://api.ivibe.live");
```

The API key is sent in the `Authorization` header for HTTPS requests and during WebSocket handshakes.

## Usage

```rust
use ivibe::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("YOUR_API_KEY", "https://api.ivibe.live");

    // Capture a custom event
    client
        .capture_event(
            "build_complete",
            json!({"project": "sdk", "status": "success"}),
            &["ci", "example"],
        )
        .await?;

    // Subscribe to live events
    client.subscribe("events").await?;

    // Export historical data
    let data = client.export_data().await?;
    println!("export size: {}", data.len());

    Ok(())
}
```

## Examples

See [`examples/basic.rs`](examples/basic.rs) for a minimal example:

```bash
cargo run --example basic
```

## Integrations

The SDK can be embedded in CI pipelines, desktop tools, or server-side agents to report custom metrics back to iVibe.live.

## License

Apache-2.0 or MIT.

# Emotion Detection Source

Rust implementation for capturing camera frames, detecting faces, classifying emotions and recording context for analytics.

## Structure

- `camera/`
  - `capture.rs`
  - `mod.rs`
  - `platform.rs`
- `wasm/`
  - `mod.rs`
  - `model.rs`
- `classifier.rs` – criticality: 9
- `context.rs` – criticality: 9
- `main.rs` – criticality: 10

Events emitted from this module feed the platform's emotion analytics features such as the `emotion_events` table and `mv_emotions` view.

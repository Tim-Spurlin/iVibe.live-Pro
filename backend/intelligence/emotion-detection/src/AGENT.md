# Agent Instructions

- **Scope**: `backend/intelligence/emotion-detection/src`
- **Purpose**: On-device emotion analysis implementation.
- **Files**:
  - `camera/` (`capture.rs`, `mod.rs`, `platform.rs`)
  - `wasm/` (`mod.rs`, `model.rs`)
  - `classifier.rs` – criticality: 9
  - `context.rs` – criticality: 9
  - `main.rs` – criticality: 10
- **Camera frame capture**: `camera::capture` streams frames via Video4Linux2 (Linux), AVFoundation (macOS) or Media Foundation (Windows).
- **Facial detection & classification**: detect faces then label smile, laugh, cry, fear and anger.
- **Context screenshot capture**: `context` records active window imagery for trigger context.
- **Trigger association**: pair emotion events with app, URL and location following the emotion analytics schema (`emotion_events`, `mv_emotions`).
- **WASM model inference**: `wasm::model` runs WebAssembly inference for cross-platform consistency.

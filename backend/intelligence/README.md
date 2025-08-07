# Intelligence Layer

The intelligence layer powers emotion detection and proximity‑based social features within the iVibe platform.

It comprises two Rust services compiled to WebAssembly for edge deployment:

- **Emotion Detection** – analyses on-device audio to infer emotional state while preserving privacy.
- **Vibe Social Engine** – uses nearby signals and shared contexts to suggest connections and interactions.

Both services communicate with the rest of the system through the Event Gateway using gRPC, enabling real-time insights without sending raw audio off-device.


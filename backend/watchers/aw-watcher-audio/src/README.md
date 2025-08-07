# aw-watcher-audio `src`

Source files for the audio watcher that captures microphone input, detects hot words, generates on-device embeddings and discards raw audio. It interfaces with PulseAudio, WASAPI, or CoreAudio and operates at 16 kHz mono with WebRTC VAD for speech gating.

## Files

| File | Criticality | Description |
| --- | --- | --- |
| `capture.rs` | 9 | Audio device interface and buffering |
| `hotword.rs` | 8 | Hot-word detection and triggering |
| `main.rs` | 10 | Entrypoint orchestrating capture, VAD and embedding |
| `privacy.rs` | 10 | Ensures raw audio is discarded and metadata attached |
| `vectorizer.rs` | 10 | On-device audio embedding generation |


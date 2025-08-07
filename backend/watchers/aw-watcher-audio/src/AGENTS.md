# Audio Watcher Agent Instructions

This directory implements privacy-preserving audio capture and vectorization for iVibe.

- Capture microphone audio at **16 kHz mono** using platform audio APIs:
  - PulseAudio on Linux
  - WASAPI on Windows
  - CoreAudio on macOS
- Apply **WebRTC Voice Activity Detection (VAD)** to gate processing and limit work to speech segments.
- Support **hot-word triggering**; only when the hot word fires should the subsequent audio be embedded.
- Generate audio embeddings **on-device** via `vectorizer.rs` and **discard raw audio** immediately.
- Attach relevant metadata (timestamp, device, hot-word tag) to each embedding event.
- Emit events that conform to the project's schema for **privacy-preserving audio vectorization**; no raw audio leaves the machine.

## Files and Criticality

| File | Criticality |
| --- | --- |
| `capture.rs` | 9 |
| `hotword.rs` | 8 |
| `main.rs` | 10 |
| `privacy.rs` | 10 |
| `vectorizer.rs` | 10 |


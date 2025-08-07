# Intelligence Engines

This directory hosts the AI and social intelligence engines that power emotion detection and vibe scoring.

## Emotion Detection
Camera frames pass through a face detector before classification into emotion vectors. Context modules capture surrounding state for each frame. Mobile builds use TensorFlow Lite models while web builds rely on WebAssembly.

## Vibe Social Engine
Devices periodically scan for nearby BLE and WiFi signals. The proximity features feed a compatibility algorithm that outputs a vibe score between 1 and 1000. Scores drive request handling and subsequent social interactions.

## Models
- **Mobile:** TensorFlow Lite models optimised for edge devices.
- **Web:** WebAssembly modules loaded for in-browser inference.

## Privacy Architecture
Processing remains on-device. Only high-level vectors or scores are transmitted, never raw images or identifiers. Communication with the broader platform occurs through gRPC calls to the Event Gateway.

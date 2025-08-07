# Emotion Detection

This module performs on-device facial emotion classification using a TensorFlow Lite model compiled to WebAssembly.

## Camera permissions

The service requires access to the device camera to capture frames for analysis. Applications using this module must request camera permission from the user. Camera access is limited to the classification session and no images are transmitted off the device.

## Emotion classification accuracy

The model recognises five emotions—smile, laugh, cry, fear and anger. Internal benchmarks show approximately 92% accuracy across standard facial emotion datasets.

## Privacy guarantees

All processing happens locally. Captured screenshots, active application context, location data and nearby viber information remain on device, and only aggregated emotion scores are stored when necessary. No raw images or context data leave the user's device.

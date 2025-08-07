# Android Mobile Client Guidelines

**Criticality:** 10 – this module handles sensitive user data.

## Services
- **TrackerService** – foreground service tracking app usage and coordinating other services.
- **EmotionService** – processes camera frames to infer emotional state.
- **LocationService** – records GPS updates for context.
- **VibeService** – captures audio, vectorises locally, and computes vibe scores.

## Permissions
The app requires the following permissions. Review carefully before adding more:
- `RECORD_AUDIO`
- `CAMERA`
- `LOCATION`
- `BLUETOOTH`
- `PACKAGE_USAGE_STATS`

## FFI
Kotlin talks to Rust through a JNI bridge in `app/src/main/java/live/ivibe/ffi`. Rust libraries in `app/libs/` perform on-device audio vectorisation and vibe scoring.

## Communication
All uploads use HTTPS to `https://api.ivibe.live/mobile`.

## Bluetooth
Vibe detection uses BLE beacon broadcasting. Use Android's `BluetoothLeAdvertiser` APIs and handle permissions gracefully.

## Storage
Audio and sensor data are vectorised on-device. Batches are stored locally and uploaded every 5 minutes.

# iVibe Android Client

## Architecture
The application is composed of multiple services managed by `TrackerService`, a foreground service that keeps the process alive and coordinates background work. `EmotionService` analyses camera frames, `LocationService` reports GPS context, and `VibeService` captures audio and computes vibe scores using on-device Rust code. BLE beacon broadcasting enables nearby vibe detection, while all outbound data is sent over HTTPS to `api.ivibe.live/mobile`.

## Permission Flow
On first run the app requests runtime permissions for `RECORD_AUDIO`, `CAMERA`, `LOCATION`, `BLUETOOTH`, and `PACKAGE_USAGE_STATS`. Each feature should check its permission before starting; deny or revoke events must disable the related service and prompt the user to re-enable it in settings.

## Rust Integration
Rust libraries compiled into `app/libs/` are accessed through JNI bindings under `app/src/main/java/live/ivibe/ffi`. Audio frames are vectorised and scored in Rust, with results stored on-device until batches are uploaded every five minutes.

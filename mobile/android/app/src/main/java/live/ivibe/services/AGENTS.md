# Android Services Instructions

- **Scope:** `mobile/android/app/src/main/java/live/ivibe/services/`
- **Purpose:** Background services powering mobile tracking features defined in the EventSchema (`location`, `vibe`, `emotion`).

## Implementation Notes
- **Foreground services:** Run each service as a `ForegroundService` with a persistent notification. Android 12+ requires an explicit notification channel and user-visible content.
- **WorkManager scheduling:** Use `WorkManager` for deferred or periodic jobs to respect system background limits.
- **Location updates:** Obtain updates from `FusedLocationProviderClient`, balancing accuracy and power usage.
- **BLE scanning:** Use `BluetoothLeScanner` for Vibe discovery; handle runtime permissions and scan filters.
- **Camera for emotion detection:** Access the camera (e.g., via `CameraX`) to capture frames for emotion analysis in `EmotionService`.
- **Battery optimization exemptions:** Request `ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS` when continuous tracking is required.
- **Notifications:** Foreground services must show ongoing notifications that satisfy Android 12+ requirements and allow users to manage settings.


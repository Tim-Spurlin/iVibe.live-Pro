# Android Main Module Agent Instructions

- **Scope:** `mobile/android/app/src/main/java/live/ivibe`
- **Architecture:** Kotlin with Jetpack Compose and Rust FFI via JNI, minimum SDK 24, per project schema.
- **Critical files:** `IVibeApplication.kt` (10/10), `MainActivity.kt` (10/10)

## Application lifecycle
- `IVibeApplication` boots the app, loads native libraries, and orchestrates long-running services in `onCreate`.
- Shut down foreground services gracefully in `onTerminate` or appropriate lifecycle hooks.

## Service management
- Services live in `services/`: `TrackerService`, `LocationService`, `EmotionService`, and `VibeService`.
- Start services from the application or activity using `startForegroundService` when background work is required.
- Ensure permission checks for RECORD_AUDIO, CAMERA, ACCESS_FINE_LOCATION, and BLUETOOTH before launching services.

## UI navigation
- `MainActivity` hosts a Jetpack Compose `NavHost` and routes to screens under `ui/dashboard`, `ui/settings`, and `ui/vibe`.
- Maintain navigation state in ViewModels and respect the back stack.

## Native library loading
- `ffi/NativeLib.kt` and `ffi/bindings.kt` wrap the Rust core.
- Load `libivibe` with `System.loadLibrary` early in `IVibeApplication` and expose safe Kotlin APIs.
- Guard against missing libraries or ABI mismatches.

## Testing
- After changes run `gradle -p mobile/android test` to verify build and unit tests.

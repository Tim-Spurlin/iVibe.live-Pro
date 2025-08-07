# Android App Module Instructions

- **Criticality:** 10/10
- **Module:** Android application module for iVibe.live
- **Language & UI:** Kotlin with Jetpack Compose for all foreground screens
- **Background Services:** Implements the Mobile Tracking Suite schema to capture app usage, location and audio context through foreground and background services
- **FFI:** Uses JNI bindings to Rust libraries for emotion detection and audio processing
- **Permissions:** Handle `RECORD_AUDIO`, `CAMERA`, `ACCESS_FINE_LOCATION`, and `BLUETOOTH` with runtime checks and user prompts
- **ProGuard/R8:** Configure rules to retain FFI bridge classes and service entry points
- **SDK Levels:** `minSdk` 24, `targetSdk` 34
- **Architecture Notes:** Compose UI interacts with services via `ViewModel` and `Service`/`Worker` patterns

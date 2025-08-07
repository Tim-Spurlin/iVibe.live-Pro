# Android Agent Instructions

- **Criticality:** 9/10
- **Purpose:** Android application
- **Files:** app/build.gradle.kts, AndroidManifest.xml, MainActivity.kt, IVibeApplication.kt
- **Services:** TrackerService, LocationService, EmotionService, VibeService
- **FFI:** Rust integration via JNI for performance
- **Min SDK:** 24 (Android 7.0)
- **Permissions:** RECORD_AUDIO, CAMERA, ACCESS_FINE_LOCATION, BLUETOOTH

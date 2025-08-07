# iVibe Android Main Module

This directory holds the core Android application code for iVibe.

## Contents
- `IVibeApplication.kt` – Application entry point responsible for lifecycle and native library setup (**criticality 10**).
- `MainActivity.kt` – Jetpack Compose activity hosting navigation among screens (**criticality 10**).
- `ffi/`
  - `NativeLib.kt` – loads the Rust library and exposes Kotlin wrappers.
  - `bindings.kt` – JNI bindings to the Rust core.
- `services/`
  - `TrackerService.kt`
  - `LocationService.kt`
  - `EmotionService.kt`
  - `VibeService.kt`
- `ui/`
  - `dashboard/`
  - `settings/`
  - `vibe/`
- `utils/`
  - `.gitkeep` – placeholder for future utilities.

The module follows the project's Android architecture using Kotlin, Jetpack Compose, and Rust FFI.

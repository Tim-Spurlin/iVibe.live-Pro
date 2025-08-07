# Android App Module

This directory contains the Android application module for iVibe.live. It integrates with the Mobile Tracking Suite to capture on-device activity and context.

## Contents
- `build.gradle.kts` – module build script (**criticality: 10**)
- `libs/`
  - `.gitkeep` – placeholder to track the directory
- `src/main/` – application sources
  - `AndroidManifest.xml`
  - `java/live/ivibe/`
    - `ffi/`
    - `services/`
    - `ui/`
    - `utils/`

## SDK Requirements
- **Minimum SDK:** 24
- **Target SDK:** 34

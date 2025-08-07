# Android Background Services

This directory contains the Android services that enable iVibe's mobile tracking features.

## Files

| File | Criticality | Description |
| --- | --- | --- |
| `EmotionService.kt` | 9 | Uses the camera to derive emotion data for upload. |
| `LocationService.kt` | 9 | Streams GPS updates and handles geofencing. |
| `TrackerService.kt` | 10 | Coordinates WorkManager tasks and sensor fusion. |
| `VibeService.kt` | 10 | Scans nearby devices via Bluetooth Low Energy to detect Vibes. |

All services operate as foreground services with persistent notifications on Android 12+ and request battery optimization exemptions when needed.

# Mobile

This directory hosts the mobile applications for the iVibe platform. Android is implemented in Kotlin, and an iOS Swift application is planned.

## Architecture
- Communicates with the iVibe API over HTTPS.
- Uses Bluetooth to detect nearby Vibes.
- Current subdirectory: `android/` for the Kotlin implementation.

## Permissions
The mobile apps require the following permissions:
- Location for proximity features.
- Camera for capturing visual context.
- Microphone for audio-based interactions.
- Bluetooth for Vibe detection.

## Build Process
### Android
1. Install Android Studio and required SDKs.
2. From the `mobile/android` directory, run `./gradlew assembleDebug` to build a debug APK.
3. Use `./gradlew installDebug` to deploy the app to a connected device or emulator.

### iOS
An iOS Swift implementation is planned. Build instructions will be added once development begins.


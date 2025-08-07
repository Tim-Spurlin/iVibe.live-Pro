# iVibe Android App

## Build Instructions
1. Install Android Studio with the Android SDK and NDK.
2. Ensure JDK 17 or higher is available.
3. Clone the repository and open `mobile/android` in Android Studio or run `gradle assembleDebug` from the command line.
4. Connect a device or start an emulator to run the debug build.

## Signing Process
1. Generate a release keystore:
   ```bash
   keytool -genkey -v -keystore ivibe.keystore -alias ivibe -keyalg RSA -keysize 2048 -validity 10000
   ```
2. Store the keystore and passwords outside version control.
3. Configure the `signingConfigs` block in `app/build.gradle.kts` and reference properties from `gradle.properties` or `local.properties`.
4. Build the signed release artifact with `gradle assembleRelease` or `gradle bundleRelease`.

## Play Store Deployment
1. In Google Play Console, create a new application and enable Play App Signing.
2. Upload the generated `*.aab` bundle from the release build.
3. Provide required metadata: description, screenshots, content rating, and privacy policy.
4. Roll out the release to internal, closed, or production tracks as needed.

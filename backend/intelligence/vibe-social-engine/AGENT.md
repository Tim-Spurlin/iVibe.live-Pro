# Vibe Social Engine Agent Instructions

- **Criticality:** 10/10
- **Purpose:** Proximity detection and compatibility scoring

## Files
- `src/main.rs`
- `src/proximity/bluetooth.rs`
- `src/proximity/wifi.rs`
- `src/scoring/algorithm.rs`
- `src/social/requests.rs`
- `src/social/privacy.rs`
- `src/sync.rs`

## Detection
- Bluetooth LE beacons
- WiFi Direct

## Score Calculation
Scores range from 1–1000 based on emotions, interests, and activities.

## Privacy
- Encrypted identifiers
- Sync only when nearby

## Data Deletion
Delete data on request denial or block.


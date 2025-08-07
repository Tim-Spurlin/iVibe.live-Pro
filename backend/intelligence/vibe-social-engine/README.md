# Vibe Social Engine

The Vibe Social Engine pairs proximity sensing with a compatibility score to foster ambient social discovery.

## Vibe Scoring Algorithm
The engine calculates a score from **1–1000** using a blend of emotional similarity, overlapping interests, and recent activities.
Each factor contributes to a weighted sum that reflects overall vibe alignment.

## Proximity Detection Range
Devices broadcast and scan using **Bluetooth LE beacons** and **WiFi Direct**. In practice, detection is limited to roughly
10 meters for Bluetooth and up to 30 meters for WiFi, depending on the environment. Synchronization occurs only within this
range to ensure encounters are genuinely nearby.

## Privacy-Preserving Sync
Identifiers are encrypted, and data exchange happens only after mutual proximity is confirmed. If a request is denied or a
user blocks another device, the corresponding data is immediately deleted. Synchronization halts as soon as devices move out of
range.


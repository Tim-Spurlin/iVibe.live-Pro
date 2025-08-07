# Watcher Development Guide

This directory contains data capture modules with **criticality 10**.

## Guidelines
- Each watcher captures its platform-specific events and forwards them to the Event Gateway via gRPC.
- Batch events and send when **512 events** are collected or **5 seconds** have elapsed.
- Implement the `EventBatch` protocol from the shared protobuf definitions.
- Use gRPC over TLS, targeting port **50051**.
- Authentication must use **JWT tokens** issued by Keycloak.

## Platform Notes
- **Window watcher:** utilise DBus on Linux, Win32 APIs on Windows, and Cocoa on macOS.
- **Terminal watcher:** hook into user shells to observe executed commands.
- **Audio watcher:** capture raw PCM frames for later vectorisation.

These requirements apply to all subdirectories within `watchers`.

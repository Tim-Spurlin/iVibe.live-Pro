# Watchers

## Architecture
Watchers capture platform-specific activity and transmit it to the Event Gateway.

- Events are buffered and dispatched when 512 items accumulate or after 5 seconds.
- Batches conform to the shared `EventBatch` protobuf and are sent via gRPC over TLS on port 50051.
- Authentication uses JWT tokens issued by Keycloak.

## Event Schema
Each `Event` in a batch includes:

- `event_id` – unique identifier (uuid4)
- `user_id` – watcher owner (uuid4)
- `timestamp` – RFC3339
- `source` – e.g. `window`, `kdevelop`, `terminal`, `audio`
- `project` – optional project association
- `language` – programming language where relevant
- `payload` – watcher-specific fields such as window title, command, or audio metadata

## Platform-Specific Implementations
- **aw-watcher-window** – tracks active windows using DBus (Linux), Win32 (Windows), or Cocoa (macOS).
- **aw-watcher-kdevelop** – KDevelop plugin emitting editor events for file edits, builds, and debugging sessions.
- **aw-watcher-terminal** – hooks into supported shells to record executed commands and exit statuses.
- **aw-watcher-audio** – records PCM audio frames for downstream vectorisation.

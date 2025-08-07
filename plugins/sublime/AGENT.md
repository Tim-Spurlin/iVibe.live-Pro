# Sublime Text Plugin Agent

- **Criticality:** 8/10
- **Purpose:** Capture Sublime Text editing activity and forward events to the iVibe backend.

## Sublime Text API usage
- Relies on the `sublime_plugin.EventListener` API to observe editor events.
- Hooks `on_activated_async` and `on_post_save_async` for focus and save notifications.

## View activation tracking
- Each time a view is activated, gather file path and language from `view.file_name()` and `view.settings().get('syntax')`.

## Project detection
- Determine the active project via `sublime.active_window().project_data()` or `window.folders()` and attach it to event payloads.

## HTTP event transmission
- Transmit JSON events with source `"sublime"` to the local iVibe endpoint (e.g., `http://localhost:3030/events`).

## Package Control submission
- Ensure a unique package name, semantic version tags, README and LICENSE files, and tagged GitHub releases before submitting to Package Control.

## Schema reference
- The global event schema enumerates **Sublime Text** as a supported editor, so emitted events should include `source: "sublime"`.

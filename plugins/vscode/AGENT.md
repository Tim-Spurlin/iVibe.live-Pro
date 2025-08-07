# VS Code Extension Agent

- **Criticality:** 9/10
- **Purpose:** Capture VS Code activity for iVibe.live
- **VS Code Extension API Usage:**
  - Monitor workspace events (`workspace.onDidOpenTextDocument`, `onDidSaveTextDocument`, `onDidChangeWorkspaceFolders`)
  - Track file open/save operations
  - Detect debug sessions via `debug.onDidStartDebugSession` and `debug.onDidTerminateDebugSession`
  - Hook into language servers and LSP diagnostics
  - Forward events to the local iVibe gateway using HTTPS requests
- **Activation Events:**
  - `"*"` – activate on VS Code startup
  - `onCommand:ivibe.start` – manual activation command
- **Contribution Points:**
  - `commands` for manual event dispatch and configuration
  - `configuration` for gateway endpoint and user preferences
  - `languages` and `debuggers` to integrate with language tools
- **Event Schema:** Events must follow the IDE plugin specification defined in the project’s root `AGENTS.md`.
- **Communication:** All events are batched and sent to `localhost` API endpoint for processing.

# Plugin Suite Guidelines

These editor integrations carry a criticality rating of **8**, reflecting their role in reliably capturing developer activity across more than 100 editors.

## Functionality
- Record coding time, file modifications, and editor errors.
- Operate transparently without capturing file contents unless configured.

## Communication
- Emit structured events to the local iVibe watcher when available.
- Fallback to sending events directly to the configured API endpoint.

## Installation
- **VS Code** and **IntelliJ**: install from their official marketplaces.
- **Neovim** and **Sublime Text**: install manually as described in each plugin's README.

## Configuration
- Set an API key and event endpoint.
- Adjust privacy options such as excluding directories or error details.
- Configuration is stored locally per editor.

## Languages
- `vscode/` – TypeScript
- `intellij/` – Kotlin
- `neovim/` – Lua
- `sublime/` – Python


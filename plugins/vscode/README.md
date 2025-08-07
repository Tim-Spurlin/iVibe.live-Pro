# VS Code Extension

This directory hosts the iVibe.live VS Code extension. The extension uses the VS Code Extension API to monitor workspace activity and forward events to the local iVibe gateway.

## Files
- `README.md` – overview and usage notes
- `package.json` (criticality: 9) – extension metadata, activation events and contribution points
- `src/extension.ts` – entry point implementing event capture
- `tsconfig.json` – TypeScript configuration

## Functionality
- Tracks file open/save and workspace changes
- Detects debug sessions and interacts with language servers
- Sends captured events to `localhost` for ingestion following the IDE plugin schema

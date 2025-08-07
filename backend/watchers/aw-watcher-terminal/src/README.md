# aw-watcher-terminal: source

This folder contains the terminal command tracking logic for ActivityWatch.
It installs shell hooks, intercepts commands with DEBUG traps, captures stderr,
classifies errors, detects fixes, and forwards events using the terminal
intelligence schema.

## Layout

- shell/
  - bash.rs
  - fish.rs
  - zsh.rs
  - powershell.rs
  - mod.rs
- command_parser.rs – criticality: 9
- error_detection.rs – criticality: 9
- main.rs – criticality: 10

These components work together to observe user commands and transmit structured
events for downstream analysis.

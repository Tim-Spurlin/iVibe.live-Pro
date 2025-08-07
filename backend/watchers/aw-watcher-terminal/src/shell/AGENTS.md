# AGENT instructions for shell integrations

This folder hosts modules that embed hooks into supported shells so `aw-watcher-terminal` can observe commands uniformly across platforms.

## Integration specifics

- **`bash.rs`** – writes a snippet to the user's `.bashrc` that augments `PROMPT_COMMAND`. The snippet records the last command, its exit status, and the time it took to run.
- **`zsh.rs`** – injects functions into `.zshrc` using `preexec` and `precmd` to capture command text, exit codes, and duration.
- **`fish.rs`** – installs a wrapper for `fish_prompt` under `~/.config/fish/functions` to emit command metadata.
- **`powershell.rs`** – appends a custom `prompt` function to `$PROFILE` so commands and timing information are reported.
- All modules send events to `/tmp/ivibe_terminal.sock` using Unix sockets. On platforms that lack Unix sockets, use the cross‑platform abstraction from the schema to maintain compatibility.
- Shared logic such as socket communication and shell detection lives in `mod.rs`.

## Developer notes

1. Hooks must be idempotent: before writing to rc files, check for existing snippets to avoid duplicates.
2. Capture and transmit: command text, exit code, and execution duration for each invocation.
3. Do not disrupt existing shell behaviour; ensure hooks degrade gracefully if the socket is unavailable.
4. Follow the schema's cross‑platform shell support guidelines when adding new shells.


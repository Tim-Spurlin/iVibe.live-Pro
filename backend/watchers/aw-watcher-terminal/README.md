# aw-watcher-terminal

`aw-watcher-terminal` records shell commands and forwards activity events over a Unix socket to the Event Gateway.

## Shell integration

1. Build the watcher:
   ```bash
   cargo build --release
   ```
2. Start the watcher so it listens on a local Unix domain socket.
3. Hook the watcher into your shell so each command is reported:
   - **Bash** (`~/.bashrc`):
     ```bash
     PROMPT_COMMAND='aw-watcher-terminal hook "$PROMPT_COMMAND"'
     ```
   - **Zsh** (`~/.zshrc`):
     ```zsh
     precmd() { aw-watcher-terminal hook "$precmd_functions"; }
     ```
   - **Fish** (`config.fish`):
     ```fish
     function fish_prompt
         aw-watcher-terminal hook $status $CMD_DURATION (pwd)
     end
     ```
   - **PowerShell** (`Microsoft.PowerShell_profile.ps1`):
     ```powershell
     function prompt {
         aw-watcher-terminal hook
         & $origPrompt
     }
     ```
Each hook sends the executed command, exit code, duration, stderr output, and working directory through the watcher’s socket.

## Privacy considerations

Command capture can include sensitive information. Events are kept on the local machine and sent only over the Unix socket to the Event Gateway. Review shell history before enabling, and configure your shell to disable or filter capture for specific directories or commands if needed.

## Error classification logic

Commands finishing with a non‑zero exit code are tagged as `unresolved`. When a subsequent command resolves the issue (exit code `0` or a recognised fix), the original event is reclassified as `fixed`. Unresolved events remain flagged until corrected.

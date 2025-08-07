# aw-watcher-terminal Agent

- **Criticality:** 10/10
- **Purpose:** Shell command tracking and error detection
- **Files:**
  - src/main.rs
  - src/command_parser.rs
  - src/error_detection.rs
  - src/shell/bash.rs
  - src/shell/zsh.rs
  - src/shell/fish.rs
  - src/shell/powershell.rs
- **Captures:** Commands, exit codes, duration, stderr, working directory
- **Classification:** Errors tagged as fixed vs unresolved
- **Shell hooks:**
  - Bash: `PROMPT_COMMAND`
  - Zsh: `precmd`
  - Fish: `fish_prompt`
  - PowerShell: `prompt`
- **Communication:** Unix socket → Event Gateway

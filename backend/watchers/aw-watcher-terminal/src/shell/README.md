# Shell integrations

This directory contains shell-specific integration modules for `aw-watcher-terminal`. Each module hooks into its corresponding shell to capture command executions and report them to the watcher. The captured data includes command text, exit codes, and execution duration. Events are forwarded via the Unix socket at `/tmp/ivibe_terminal.sock`, enabling cross-platform shell support defined in the iVibe schema.

## Files

| File | Criticality | Hook/Feature |
| --- | --- | --- |
| `bash.rs` | 10 | Extends `PROMPT_COMMAND` to track each command |
| `fish.rs` | 10 | Wraps `fish_prompt` to forward command info |
| `zsh.rs` | 10 | Uses `preexec`/`precmd` hooks |
| `powershell.rs` | 10 | Overrides the PowerShell `prompt` function |
| `mod.rs` | 8 | Shared helpers and shell detection |


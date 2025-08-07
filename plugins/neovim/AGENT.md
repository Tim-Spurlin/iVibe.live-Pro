# Neovim Plugin Guidelines

- **Scope:** Neovim integration for iVibe.
- **Lua API:** Use Neovim's Lua APIs via `vim.api` and `vim.fn` within modules under `lua/ivibe`.
- **Autocommands:** Register `BufEnter` and `BufWrite` autocmds to capture file open and save events.
- **LSP Integration:** Query active `vim.lsp` clients to enrich events with project and language metadata.
- **Terminal Monitoring:** Watch `TermOpen` and terminal buffer updates to record shell commands.
- **Async Transmission:** Send events to the local iVibe endpoint using non-blocking calls such as `vim.loop.spawn` or `vim.fn.jobstart`.
- **Schema Alignment:** Events must follow the editor portion of the `EventSchema` described in `/AGENTS.md`.
- **Installation:** With [lazy.nvim](https://github.com/folke/lazy.nvim):

```lua
{
  "ivibe/neovim",
  config = function()
    require("ivibe").setup()
  end,
}
```

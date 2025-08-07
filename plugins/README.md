# iVibe Editor Plugins

Official iVibe plugins bring activity tracking to over 100 editors. This directory currently hosts first‑party integrations for:

- **VS Code**
- **IntelliJ IDEA**
- **Neovim**
- **Sublime Text**

## Installation

### VS Code
1. Open the Extensions view (`Ctrl+Shift+X`).
2. Search for `iVibe` and install the extension from the Marketplace.
3. Configure your API key and endpoint in the extension settings.

### IntelliJ IDEA
1. Go to **Settings → Plugins**.
2. Search for `iVibe` in the JetBrains Marketplace and install.
3. Restart the IDE and enter your API key in **Tools → iVibe**.

### Neovim
1. Install manually or via a plugin manager:
   ```lua
   use { 'ivibe/ivibe-live', rtp = 'plugins/neovim' }
   ```
2. Run `:lua require('ivibe').setup{ api_key = 'YOUR_KEY', endpoint = 'https://localhost:5600' }`.

### Sublime Text
1. Copy `ivibe.py` into your `Packages` directory or add the repository to Package Control.
2. Set your API key and endpoint in `Preferences → Package Settings → iVibe`.

For advanced configuration and troubleshooting, see the README in each plugin's subdirectory.


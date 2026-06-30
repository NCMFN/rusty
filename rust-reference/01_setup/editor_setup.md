# Editor & IDE Setup

## Overview

Rust development is excellent in any modern editor with **rust-analyzer**, the official language server. This guide covers setup for popular options.

---

## VS Code (Recommended for Beginners)

### Installation

1. **Install VS Code** from [code.visualstudio.com](https://code.visualstudio.com/)
2. **Open Extensions** (Ctrl+Shift+X / Cmd+Shift+X)
3. **Search** for "rust-analyzer"
4. **Install** the official extension by The Rust Foundation

### Essential Extensions

| Extension | Purpose |
|-----------|----------|
| **rust-analyzer** | Language server (required) |
| **Even Better TOML** | Syntax highlighting for Cargo.toml |
| **Crates** | Inline version display for dependencies |
| **Error Lens** | Show errors inline |
| **Better Comments** | Highlight TODOs and FIXMEs |

### Configuration

Create `.vscode/settings.json` in your project:

```json
{
  "[rust]": {
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.codeActionsOnSave": {
      "source.fixAll.clippy": true
    }
  },
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.checkOnSave.extraArgs": ["--all-targets"],
  "rust-analyzer.diagnostics.disabled": [],
  "rust-analyzer.inlayHints.typeHints.enable": true,
  "rust-analyzer.inlayHints.parameterHints.enable": true,
  "rust-analyzer.hover.documentation.enable": true,
  "editor.inlayHints.fontFamily": "monospace",
  "editor.inlayHints.fontSize": 12
}
```

### Key Shortcuts

| Shortcut | Action |
|----------|--------|
| `F12` | Go to definition |
| `Ctrl+Shift+I` | Go to implementation |
| `Hover` | Show documentation |
| `Ctrl+.` | Quick fixes |
| `F2` | Rename symbol |
| `Ctrl+K Ctrl+0` | Fold all |
| `Ctrl+Shift+Enter` | Format document |

### Running & Debugging

Create `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Rust: build",
      "type": "cargo",
      "command": "build",
      "problemMatcher": ["$rustc"],
      "group": {
        "kind": "build",
        "isDefault": true
      }
    },
    {
      "label": "Rust: run",
      "type": "cargo",
      "command": "run",
      "problemMatcher": ["$rustc"],
      "group": {
        "kind": "build"
      }
    },
    {
      "label": "Rust: test",
      "type": "cargo",
      "command": "test",
      "problemMatcher": ["$rustc"],
      "group": {
        "kind": "test"
      }
    }
  ]
}
```

Run with **Ctrl+Shift+B** or **Terminal** → **Run Task**.

---

## IntelliJ IDEA / CLion

### Setup

1. **Open Settings** (Ctrl+Alt+S / Cmd+,)
2. **Go to Plugins**
3. **Search** for "Rust"
4. **Install** the official Rust plugin
5. **Restart** IDE

### Features

- Built-in Cargo support
- Test runner UI
- Debugger integration
- Code inspections
- Quick-fix suggestions

### Configuration

Settings > Languages & Frameworks > Rust:

- **Toolchain location**: Auto-detected from `~/.rustup`
- **Use separate build output path**: Checked
- **Reformat code on Save**: Enabled
- **Run clippy on Save**: Optional

### Running Tests

1. Click the green play icon next to test function
2. Or: **Ctrl+Shift+F10** on test
3. Results appear in test runner window

---

## Neovim

### Setup with nvim-lspconfig

Install vim-plug or packer:

**packer.nvim** example in `~/.config/nvim/init.lua`:

```lua
return require('packer').startup(function(use)
  use 'wbthomason/packer.nvim'
  
  -- LSP Configuration
  use 'neovim/nvim-lspconfig'
  use 'hrsh7th/nvim-cmp'
  use 'hrsh7th/cmp-nvim-lsp'
  use 'L3MON4D3/LuaSnip'
  
  -- Treesitter (syntax highlighting)
  use { 'nvim-treesitter/nvim-treesitter', run = ':TSUpdate' }
end)
```

### LSP Configuration

Add to `~/.config/nvim/init.lua`:

```lua
local lspconfig = require('lspconfig')

lspconfig.rust_analyzer.setup {
  settings = {
    ["rust-analyzer"] = {
      checkOnSave = {
        command = "clippy",
        extraArgs = { "--all-targets" }
      },
      inlayHints = {
        enable = true,
      },
    },
  },
}

-- Keybindings
local opts = { noremap=true, silent=true }
vim.keymap.set('n', 'gd', vim.lsp.buf.definition, opts)
vim.keymap.set('n', 'K', vim.lsp.buf.hover, opts)
vim.keymap.set('n', '<leader>rn', vim.lsp.buf.rename, opts)
vim.keymap.set('n', '<leader>ca', vim.lsp.buf.code_action, opts)
```

### Building in Neovim

Add to `init.lua`:

```lua
vim.keymap.set('n', '<leader>cb', ':!cargo build<CR>', opts)
vim.keymap.set('n', '<leader>cr', ':!cargo run<CR>', opts)
vim.keymap.set('n', '<leader>ct', ':!cargo test<CR>', opts)
```

---

## Helix

Helix has Rust support built-in.

### Configuration

Create or edit `~/.config/helix/languages.toml`:

```toml
[[language]]
name = "rust"
scope = "source.rust"
injection-regex = "rust"
file-types = ["rs"]

[language.language-server]
command = "rust-analyzer"

[language.debugger]
command = "lldb-vscode"

[language.auto-format]
command = "rustfmt"
args = ["--edition", "2021"]
```

### Key Bindings

Edit `~/.config/helix/config.toml`:

```toml
[keys.normal]
space.c = { b = ":sh cargo build", r = ":sh cargo run", t = ":sh cargo test" }
```

---

## Vim (Classic)

### Install rust.vim

Using vim-plug in `~/.vimrc`:

```vim
Plug 'rust-lang/rust.vim'
Plug 'neovim/nvim-lspconfig'  " Neovim only
```

Then `:PlugInstall`.

### Syntax Highlighting

Automatic with rust.vim plugin.

### Formatting on Save

Add to `~/.vimrc`:

```vim
autocmd BufWritePre *.rs :%!rustfmt
```

---

## Remote Development

### SSH with VS Code

1. Install **Remote - SSH** extension
2. Open Command Palette (Ctrl+Shift+P)
3. **Remote-SSH: Connect to Host**
4. Enter SSH connection string
5. Install Rust and rust-analyzer on remote

### Codespaces

GitHub Codespaces include Rust out of the box.

1. Open your repo on GitHub
2. Click **Code** → **Codespaces** → **Create codespace on main**
3. VS Code opens in browser with Rust ready

---

## Troubleshooting

### rust-analyzer Not Working

```bash
# Ensure rust-analyzer is installed
rustup component add rust-analyzer

# Reinstall it
rustup component remove rust-analyzer
rustup component add rust-analyzer
```

### Slow Completions

Reduce indexed crates in VS Code settings:

```json
{
  "rust-analyzer.checkOnSave.extraArgs": ["--lib", "--tests"]
}
```

### "Unresolved import" Error

1. Ensure dependencies are in `Cargo.toml`
2. Run `cargo build` once
3. Reload window (Ctrl+R in VS Code)

---

## Key Takeaways

✅ **VS Code + rust-analyzer** is the best choice for beginners.

✅ **IntelliJ/CLion** has excellent built-in Rust support.

✅ **Neovim with nvim-lspconfig** gives a modern editing experience.

✅ **All setups use the same language server** (rust-analyzer) for consistency.

✅ **Format on save** and **clippy on check** are recommended.

---

**Next:** [Your First Project](first_project.md) — Create and run your first Rust program.
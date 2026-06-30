# Installation & Setup

## Installing Rust with rustup

### Linux & macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts. When asked about the installation, press `1` and Enter for default.

Then activate the environment:

```bash
source $HOME/.cargo/env
```

### Windows

1. Download [rustup-init.exe](https://rustup.rs/) from rustup.rs
2. Run the installer
3. Press `1` for default installation
4. Restart your terminal

### Verify Installation

```bash
rustc --version
# rustc 1.77.0 (aaac8296a 2024-03-30)

cargo --version
# cargo 1.77.0 (e52e36bf9 2024-03-30)

rustup --version
# rustup 1.27.0 (2024-03-02)
```

If these commands work, you're ready to go!

### Updating Rust

```bash
# Keep everything up to date
rustup update
```

Run this periodically to get the latest stable release.

---

## Uninstalling Rust

If you ever need to remove Rust:

```bash
rustup self uninstall
```

This removes Rust entirely, including the `.cargo` and `.rustup` directories.

---

## Troubleshooting Installation

### Problem: `rustc: command not found`

**Solution**: Add Cargo to your PATH.

```bash
# Linux/macOS
export PATH="$HOME/.cargo/bin:$PATH"

# Add to ~/.bashrc or ~/.zshrc to make permanent
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Problem: Permission denied on rustup-init.exe (Windows)

**Solution**: Run as Administrator.

### Problem: Slow download on rustup install

**Solution**: Use a mirror. Edit `~/.cargo/config.toml`:

```toml
[source.crates-io]
replace-with = 'mirror'

[source.mirror]
registry = "https://github.com/rust-lang/crates.io-index"
```

---

## Key Takeaways

✅ **Installation is straightforward** via rustup on all platforms.

✅ **One command** (`rustup update`) keeps everything current.

✅ **Uninstall is clean** with `rustup self uninstall`.

✅ **Verification is simple** with `rustc --version` and `cargo --version`.

---

**Next:** [Toolchain Management](toolchain.md) — Stable, beta, nightly, and targets.
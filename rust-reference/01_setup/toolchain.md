# Rust Toolchain Management

## Understanding Toolchains

Rust has three release channels:

### 1. Stable (Recommended)
**Every 6 weeks**, a new stable version releases. This is the default.

```bash
rustup default stable
rustc --version
# rustc 1.77.0 (aaac8296a 2024-03-30)
```

### 2. Beta
One version ahead of stable. Testing ground for the next release.

```bash
rustup default beta
rustc --version
# rustc 1.78.0-beta.1 (2024-04-13)
```

### 3. Nightly
Daily builds with experimental features. Unstable API.

```bash
rustup default nightly
rustc --version
# rustc 1.79.0-nightly (86e3b7e24 2024-04-14)
```

---

## Switching Toolchains

### Globally

```bash
# Switch to stable
rustup default stable

# Switch to nightly
rustup default nightly

# Check current default
rustup show active-toolchain
# stable-x86_64-unknown-linux-gnu (default)
```

### Per-Project

Create `rust-toolchain.toml` in project root:

```toml
[toolchain]
channel = "nightly"
```

Or use command:

```bash
cd my_project
rustup override set nightly
```

### Temporarily

```bash
# Use nightly for one command
rustup run nightly cargo build

# Or with +
cargo +nightly build
```

---

## Installing Specific Versions

### Install a Specific Version

```bash
# Install Rust 1.70.0
rustup install 1.70.0

# Use it globally
rustup default 1.70.0

# Use it for a project
rustup override set 1.70.0
```

### List Installed Toolchains

```bash
rustup toolchain list
# stable-x86_64-unknown-linux-gnu (default)
# nightly-x86_64-unknown-linux-gnu
# 1.70.0-x86_64-unknown-linux-gnu
```

### Remove a Toolchain

```bash
rustup uninstall 1.70.0
```

---

## Components

Toolchains contain multiple components:

### Core Components

| Component | Purpose |
|-----------|----------|
| `rustc` | Rust compiler |
| `cargo` | Package manager & build tool |
| `rust-std` | Standard library |
| `rust-docs` | Documentation |

### Additional Components

```bash
# List available components
rustup component list

# List installed components
rustup component list --installed
```

### Installing Components

```bash
# rustfmt (code formatter)
rustup component add rustfmt

# clippy (linter)
rustup component add clippy

# rust-analyzer (language server)
rustup component add rust-analyzer

# For nightly
rustup component add rustfmt --toolchain nightly
```

### Using Components

```bash
# Format code
cargo fmt

# Check with clippy
cargo clippy

# Generate docs
cargo doc
```

---

## Compilation Targets

Cross-compile to different platforms:

### Viewing Targets

```bash
# List all available targets
rustup target list

# Show installed targets
rustup target list --installed
# x86_64-unknown-linux-gnu (installed)
```

### Installing Targets

```bash
# WebAssembly
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi

# ARM (Embedded)
rustup target add thumbv7em-none-eabihf  # ARM Cortex-M
rustup target add aarch64-unknown-linux-gnu  # ARM 64-bit

# RISC-V (for RISC-V systems)
rustup target add riscv64gc-unknown-linux-gnu

# Apple Silicon (M1/M2/M3)
rustup target add aarch64-apple-darwin

# Windows MSVC
rustup target add x86_64-pc-windows-msvc
```

### Building for a Target

```bash
# Default target (your host machine)
cargo build

# Specific target
cargo build --target wasm32-unknown-unknown

# Release for embedded
cargo build --target thumbv7em-none-eabihf --release
```

---

## Common Toolchain Configurations

### Embedded Development

Create `rust-toolchain.toml`:

```toml
[toolchain]
channel = "stable"
targets = ["thumbv7em-none-eabihf"]
components = ["rust-src"]
```

### WebAssembly Development

```bash
rustup target add wasm32-unknown-unknown
rustup component add rust-src

# Install wasm-pack
cargo install wasm-pack
```

### Full Nightly Setup

```toml
[toolchain]
channel = "nightly"
components = ["rustfmt", "clippy", "rust-analyzer", "rust-src"]
targets = ["wasm32-unknown-unknown", "thumbv7em-none-eabihf"]
```

---

## Keeping Rust Updated

### Check for Updates

```bash
rustup check
# This is a no-op, as rustup self-updates at startup
```

### Update Everything

```bash
# Update all installed toolchains
rustup update

# Update just stable
rustup update stable

# Update nightly
rustup update nightly
```

### Automatic Updates

rustup checks for updates automatically when you run `cargo` or `rustc`.

---

## Nightly-Only Features

Some features require nightly Rust. Use `#![feature(...)]` at the top of your crate:

```rust
#![feature(box_syntax)]
#![feature(adt_const_params)]

fn main() {
    // Use nightly features here
}
```

Then build with:

```bash
cargo +nightly build
```

**Warning**: Nightly features can change or disappear. Use only when necessary.

---

## Debugging Toolchain Issues

### Show Everything

```bash
rustup show
# Default host: x86_64-unknown-linux-gnu
# rustup home:  /home/user/.rustup
#
# installed toolchains:
# stable-x86_64-unknown-linux-gnu (default)
# nightly-x86_64-unknown-linux-gnu
#
# installed targets for active toolchain:
# x86_64-unknown-linux-gnu
# wasm32-unknown-unknown
#
# active toolchain:
# stable-x86_64-unknown-linux-gnu (overridden by '/path/project/rust-toolchain.toml')
```

### Show Full Version Info

```bash
rustc --version --verbose
# rustc 1.77.0 (aaac8296a 2024-03-30)
# binary: rustc
# commit-hash: aaac8296a
# commit-date: 2024-03-30
# host: x86_64-unknown-linux-gnu
# release: 1.77.0
# LLVM version: 17.0.6
```

---

## Key Takeaways

✅ **Three channels**: Stable (recommended), Beta (testing), Nightly (experimental).

✅ **Switch globally or per-project** with `rustup default` or `rust-toolchain.toml`.

✅ **Install components** like `rustfmt`, `clippy`, `rust-analyzer`.

✅ **Cross-compile** to different targets (WebAssembly, ARM, etc.).

✅ **Keep updated** with `rustup update`.

✅ **Use nightly judiciously** only for necessary experimental features.

---

**Next:** [Editor Setup](editor_setup.md) — Configure VS Code, IntelliJ, or Vim.
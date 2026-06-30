# The Rust Ecosystem

## Overview

Rust's power comes not just from the language, but from its exceptional tooling and vibrant ecosystem. This section covers the core tools, package management, and community resources that make Rust development productive.

---

## The Toolchain: rustup

### What is rustup?

**rustup** is the Rust toolchain installer and version manager. It manages:
- Rust compiler versions (stable, beta, nightly)
- Build targets (different platforms)
- Components (rustfmt, clippy, rust-analyzer)

### Installation

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

On Windows, download from [rustup.rs](https://rustup.rs/).

### Checking Your Installation

```bash
rustc --version
# rustc 1.77.0 (aaac8296a 2024-03-30)

cargo --version
# cargo 1.77.0 (e52e36bf9 2024-03-30)

rustup show
# Default host: x86_64-unknown-linux-gnu
# Installed toolchains:
# stable-x86_64-unknown-linux-gnu (default)
# nightly-x86_64-unknown-linux-gnu
```

### Managing Toolchains

```bash
# Update to latest stable
rustup update

# Switch to nightly (for experimental features)
rustup default nightly

# Use nightly for a specific project
cd my_project
rustup override set nightly

# Check which toolchain is active
rustup show active-toolchain

# Install a specific version
rustup install 1.70.0
```

### Common Targets

Cross-compile to different platforms:

```bash
# See all available targets
rustup target list

# Install a target (e.g., WebAssembly)
rustup target add wasm32-unknown-unknown

# Install ARM for embedded systems
rustup target add thumbv7em-none-eabihf

# Build for a specific target
cargo build --target wasm32-unknown-unknown
```

---

## Cargo: The Package Manager

### What is Cargo?

**Cargo** is Rust's official package manager and build system. It handles:
- Project creation and structure
- Dependency management
- Building binaries and libraries
- Running tests
- Publishing to crates.io
- Managing documentation

### Creating a New Project

```bash
# Binary project
cargo new hello_world
cd hello_world

# Or library project
cargo new --lib my_library
```

### Project Structure

```
hello_world/
├── Cargo.toml        # Project manifest (like package.json)
├── Cargo.lock        # Dependency lock file (like package-lock.json)
├── src/
│   └── main.rs       # Entry point for binary
├── target/           # Compiled output (ignore this)
└── README.md
```

### Cargo.toml: The Manifest

```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"          # Rust edition (2015, 2018, or 2021)
author = "Your Name"
description = "A simple hello world"
license = "MIT"

[dependencies]            # External crates
serde = "1.0"             # With version
tokio = { version = "1.35", features = ["full"] }
tracing = "0.1"

[dev-dependencies]       # Only for tests
criteria = "0.5"

[lib]                    # Library config
path = "src/lib.rs"

[[bin]]                  # Binary config
name = "cli_tool"
path = "src/main.rs"
```

### Essential Cargo Commands

```bash
# Build (debug mode, faster compile, slower runtime)
cargo build

# Build for release (slower compile, optimized runtime)
cargo build --release

# Run directly (debug)
cargo run

# Run release version
cargo run --release

# Run with arguments
cargo run -- arg1 arg2

# Check (compile without linking, very fast)
cargo check

# Test
cargo test

# Clean build artifacts
cargo clean

# Format code
cargo fmt

# Lint with clippy
cargo clippy

# Generate documentation
cargo doc --open

# Publish to crates.io
cargo publish
```

### Adding Dependencies

```bash
# Add a dependency
cargo add serde

# Add with specific version
cargo add tokio@1.35

# Add with features
cargo add serde --features derive

# Add as dev-dependency
cargo add --dev criterion

# Remove a dependency
cargo remove serde
```

---

## Crates.io: The Package Registry

### What is crates.io?

[crates.io](https://crates.io/) is Rust's official package registry, hosting 100,000+ community-maintained libraries.

### Finding Crates

```bash
# Search on command line
cargo search serde

# Or visit https://crates.io and search
```

### Understanding Crate Versions

Rust uses **Semantic Versioning**:

```
1.2.3
│ │ │
│ │ └─ Patch (bug fixes, backwards compatible)
│ └─── Minor (new features, backwards compatible)
└───── Major (breaking changes)
```

### Version Specifiers

```toml
# Exact version
serde = "1.0.0"

# Latest patch (1.0.x)
serde = "1.0"

# Latest minor (1.x)
serde = "1"

# Minimum version
serde = ">= 1.0"

# Range
serde = "1.0..2.0"

# Caret (default)
# Allows changes that don't modify the left-most non-zero digit
serde = "^1.2.3"  # Allows 1.2.3 to 1.x.x
serde = "^0.2.3"  # Allows 0.2.3 to 0.2.x (not 0.3.0)
```

### Popular Crates

#### Serialization
- **serde**: The de facto standard for serialization
- **serde_json**: JSON serialization
- **toml**: TOML config file parsing
- **bincode**: Binary serialization

#### Async/Runtime
- **tokio**: Popular async runtime
- **async-std**: Alternative async runtime
- **smol**: Minimal async runtime
- **futures**: Core async abstractions

#### Web
- **axum**: Modern async web framework
- **actix-web**: High-performance web framework
- **rocket**: Easy-to-use web framework
- **reqwest**: HTTP client
- **hyper**: Low-level HTTP library

#### Data & Storage
- **sqlx**: Async SQL toolkit
- **diesel**: ORM for SQL
- **mongodb**: MongoDB driver
- **redis**: Redis client
- **rusqlite**: SQLite binding

#### CLI Tools
- **clap**: Command-line argument parsing
- **structopt**: Derive macros for clap
- **termcolor**: Colored terminal output
- **indicatif**: Progress bars

#### Utilities
- **regex**: Regular expressions
- **chrono**: Date/time handling
- **uuid**: UUID generation
- **rand**: Random number generation
- **lazy_static**: Lazy static initialization

#### Testing
- **criterion**: Benchmarking
- **proptest**: Property-based testing
- **mockall**: Mocking framework

---

## Development Tools

### rustfmt: Code Formatter

**Automatically format Rust code** to a consistent style.

```bash
# Format current project
cargo fmt

# Check formatting without modifying
cargo fmt -- --check
```

**Configuration** (`rustfmt.toml` in project root):

```toml
max_width = 100
hard_tabs = false
edition = "2021"
```

### Clippy: Linter

**Lint and suggest improvements** to your code.

```bash
# Run all lints
cargo clippy

# Check for potential issues
cargo clippy -- -W clippy::all
```

**Common clippy suggestions:**
- Use idiomatic patterns
- Catch performance issues
- Suggest better APIs
- Flag potential bugs

**Example warning:**

```rust
let mut vec = Vec::new();
vec.push(1);
vec.push(2);
```

**Clippy says**: "Use vec![1, 2] instead"

### rust-analyzer: Language Server

**rust-analyzer** provides IDE-like features:
- Code completion
- Go to definition
- Hover documentation
- Inline type hints
- Refactoring
- Syntax highlighting

Installed automatically with `rustup`:

```bash
rustup component add rust-analyzer
```

---

## Editor Setup

### VS Code (Recommended for Beginners)

1. Install **Rust Analyzer** extension
2. Optional: Install **Even Better TOML** for Cargo.toml
3. Optional: Install **Crates** for dependency management

**Configuration** (`.vscode/settings.json`):

```json
{
  "[rust]": {
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "rust-analyzer.checkOnSave.command": "clippy"
}
```

### IntelliJ/CLion

Built-in Rust support. Install the official Rust plugin.

### Vim/Neovim

With **nvim-lspconfig**:

```lua
require('lspconfig').rust_analyzer.setup {}
```

---

## Documentation

### Generating Project Documentation

```bash
# Generate docs and open in browser
cargo doc --open
```

### Writing Documentation

```rust
/// Calculate the sum of two numbers.
///
/// # Examples
///
/// ```
/// assert_eq!(add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Official Documentation

- [The Rust Book](https://doc.rust-lang.org/book/)
- [The Rust Reference](https://doc.rust-lang.org/reference/)
- [Standard Library Docs](https://doc.rust-lang.org/std/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

---

## Community Resources

### Forums & Discussion

- **[Rust Forum](https://users.rust-lang.org/)**: Official discussion board
- **[r/rust](https://reddit.com/r/rust)**: Reddit community
- **[Rust Discord](https://discord.gg/rust-lang)**: Real-time chat
- **[Users GitHub Discussions](https://github.com/rust-lang/rust/discussions)**: GitHub discussions

### Learning

- **[Rust by Example](https://doc.rust-lang.org/rust-by-example/)**
- **[The Rustlings Course](https://github.com/rust-lang/rustlings)**: Interactive exercises
- **[100 Exercises to Learn Rust](https://github.com/mainmatter/100-exercises-to-learn-rust)**
- **[Tour of Rust](https://tourofrust.com/)**

### Articles & Blogs

- **[Rust Blog](https://blog.rust-lang.org/)**
- **[Inside Rust](https://blog.rust-lang.org/inside-rust/)**
- **Popular bloggers**: Jon Gjengset, Niko Matsakis, etc.

### Conferences

- **[RustConf](https://rustconf.com/)**
- **[Rust Europe](https://rustfest.world/)**
- **Local meetups** (search Meetup.com)

---

## Continuous Integration

### GitHub Actions Example

```yaml
name: Rust Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --all
      - run: cargo fmt -- --check
      - run: cargo clippy -- -D warnings
```

---

## Performance Profiling

### Flamegraph

Visualize where your program spends time:

```bash
# Install
cargo install flamegraph

# Generate flamegraph
cargo flamegraph

# Opens flamegraph.svg in browser
```

### Perf (Linux)

```bash
cargo build --release
perf record -g target/release/my_program
perf report
```

---

## Key Takeaways

✅ **rustup** manages your Rust toolchain, versions, and targets.

✅ **Cargo** is your primary tool for project management, building, testing, and publishing.

✅ **crates.io** hosts 100,000+ community libraries with semantic versioning.

✅ **rustfmt** and **clippy** enforce code quality and idiomatic Rust.

✅ **rust-analyzer** provides modern IDE features in any editor.

✅ **The community** is welcoming, well-documented, and helpful.

✅ **Tooling is excellent** — Rust development is a great experience.

---

**Next:** [Installation & Setup](../01_setup/installation.md) — Get Rust running on your machine.
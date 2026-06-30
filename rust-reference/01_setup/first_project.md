# Your First Rust Project

## Creating a Project

### Binary Project (Executable)

A binary project produces a runnable executable:

```bash
cargo new hello_world
cd hello_world
```

**Project structure:**

```
hello_world/
├── Cargo.toml          # Project manifest
├── Cargo.lock          # Dependency lock file (auto-generated)
├── src/
│   └── main.rs         # Entry point
└── target/             # Compiled output (ignore this)
    └── debug/
        └── hello_world # Executable
```

### Library Project

A library is code meant to be reused by other projects:

```bash
cargo new --lib my_library
cd my_library
```

**Project structure:**

```
my_library/
├── Cargo.toml
├── src/
│   └── lib.rs          # Library entry point (no main())
└── target/
    └── debug/
        └── libmy_library.rlib
```

---

## Understanding Cargo.toml

The manifest file describes your project:

```toml
[package]
name = "hello_world"     # Project name
version = "0.1.0"       # Semantic version
edition = "2021"        # Rust edition (2015, 2018, 2021)
authors = ["You <you@example.com>"]
description = "My first Rust project"
license = "MIT"         # Optional

[dependencies]          # External crates
serde = "1.0"          # With version
tokio = { version = "1.35", features = ["full"] }  # With features

[dev-dependencies]     # Only for tests
criterion = "0.5"

# Binary configuration (if needed)
[[bin]]
name = "my_app"
path = "src/main.rs"

# Library configuration (if needed)
[lib]
name = "my_lib"
path = "src/lib.rs"
```

---

## Your First Program

### Hello, World!

Open `src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

**Run it:**

```bash
cargo run
```

**Output:**

```
   Compiling hello_world v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/hello_world`
Hello, world!
```

### A More Interesting Example

```rust
fn main() {
    let name = "Rust";
    let age = 16;
    let is_powerful = true;
    
    println!("Hello, {}!", name);
    println!("{} is {} years old", name, age);
    println!("Powerful? {}", is_powerful);
}
```

**Run:**

```bash
cargo run
```

**Output:**

```
Hello, Rust!
Rust is 16 years old
Powerful? true
```

---

## Common Cargo Commands

### Build

```bash
# Debug build (fast compile, slow runtime)
cargo build

# Release build (slow compile, fast runtime)
cargo build --release

# Check for errors without building
cargo check
```

### Run

```bash
# Run debug binary
cargo run

# Run release binary
cargo run --release

# Run with arguments
cargo run -- arg1 arg2

# Example
cargo run -- --name Alice --verbose
```

### Test

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Other Useful Commands

```bash
# Format code
cargo fmt

# Lint with clippy
cargo clippy

# Generate docs
cargo doc --open

# List dependencies
cargo tree

# Clean build artifacts
cargo clean
```

---

## Project Layout Best Practices

### Binary Project with Multiple Files

```
hello_world/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── src/
│   ├── main.rs         # Binary entry point
│   ├── lib.rs          # Shared code
│   └── utils.rs        # Helper functions
├── tests/              # Integration tests
│   └── integration_test.rs
└── target/
```

**src/main.rs:**
```rust
mod lib;  // Include lib.rs

fn main() {
    lib::greet("World");
}
```

**src/lib.rs:**
```rust
pub fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

**Run:**
```bash
cargo run
# Hello, World!
```

### Library Project with Tests

```
my_lib/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── math.rs
│   └── utils.rs
└── tests/
    └── integration_tests.rs
```

**src/lib.rs:**
```rust
pub mod math;
pub mod utils;

pub fn public_function() {
    println!("Public function");
}
```

**src/math.rs:**
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
```

**Run tests:**
```bash
cargo test
```

---

## Adding Dependencies

### Manually

Edit `Cargo.toml`:

```toml
[dependencies]
serde = "1.0"
serde_json = "1.0"
```

### Via cargo-add

```bash
# Add a dependency
cargo add serde

# Add with specific version
cargo add serde@1.0.200

# Add with features
cargo add serde --features derive

# Add as dev-dependency
cargo add --dev criterion

# Remove a dependency
cargo remove serde
```

### Example: Using serde

```bash
cargo add serde serde_json
```

**src/main.rs:**
```rust
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    
    let json = serde_json::to_string_pretty(&person).unwrap();
    println!("{}", json);
}
```

**Run:**
```bash
cargo run
```

**Output:**
```json
{
  "name": "Alice",
  "age": 30
}
```

---

## Common Patterns

### Multiple Binary Targets

Create `src/bin/`:

```
hello_world/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── lib.rs
└── src/bin/
    ├── tool1.rs        # cargo run --bin tool1
    └── tool2.rs        # cargo run --bin tool2
```

**src/bin/tool1.rs:**
```rust
use hello_world::greet;  // Use functions from lib.rs

fn main() {
    greet("Tool 1");
}
```

**Run:**
```bash
cargo run --bin tool1
cargo run --bin tool2
```

### Workspace with Multiple Crates

**Workspace root Cargo.toml:**
```toml
[workspace]
members = ["lib_a", "lib_b", "app"]
```

**Structure:**
```
my_workspace/
├── Cargo.toml
├── Cargo.lock
├── lib_a/
│   ├── Cargo.toml
│   └── src/lib.rs
├── lib_b/
│   ├── Cargo.toml
│   └── src/lib.rs
└── app/
    ├── Cargo.toml
    └── src/main.rs
```

**Commands:**
```bash
# Build all crates
cargo build

# Build specific crate
cargo build -p app

# Test all
cargo test

# Run app
cargo run -p app
```

---

## Debugging Tips

### Compiler Error Messages

Rust's error messages are detailed. Read them carefully:

```rust
fn main() {
    let x = 5;
    x = 6;  // Error
}
```

**Error:**
```
error[E0384]: cannot assign twice to immutable variable `x`
  --> src/main.rs:3:5
   |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         ^^^
```

**Fix**: Follow the compiler's suggestion:

```rust
fn main() {
    let mut x = 5;
    x = 6;  // OK
}
```

### Using println! for Debugging

```rust
fn main() {
    let value = 42;
    println!("Debug: value = {:?}", value);
}
```

### Using dbg! Macro

```rust
fn main() {
    let value = 42;
    dbg!(value);
    // [src/main.rs:3] value = 42
}
```

---

## Key Takeaways

✅ **Create projects** with `cargo new` for binaries or `cargo new --lib` for libraries.

✅ **Cargo.toml** describes your project and dependencies.

✅ **cargo run** compiles and runs your project.

✅ **cargo check** quickly checks for errors without building.

✅ **cargo add** manages dependencies from the command line.

✅ **Error messages are helpful** — read them carefully!

✅ **Use workspaces** for multi-crate projects.

✅ **Use multiple bin targets** for multiple executable programs.

---

**Next:** [Fundamentals](../02_fundamentals/variables_and_mutability.md) — Learn variables, types, and functions.
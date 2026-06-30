# Comments and Documentation

## Regular Comments

### Line Comments

Start with `//` and extend to the end of the line:

```rust
fn main() {
    // This is a comment
    let x = 5;  // x is a variable
    
    // TODO: Implement this feature
    // FIXME: This is buggy
    // HACK: Temporary workaround
}
```

### Block Comments

Start with `/*` and end with `*/`:

```rust
fn main() {
    /* This is a
       multi-line comment
       that can span many lines */
    
    let x = 5; /* inline block comment */
}
```

### Nested Block Comments

Rust supports nested block comments:

```rust
fn main() {
    /* Outer comment /* inner comment */ back to outer */
}
```

---

## Documentation Comments

Documentation comments use `///` or `//!` and generate HTML documentation via `cargo doc`.

### Item Documentation (`///`)

Documents the item that follows:

```rust
/// Adds two numbers together.
///
/// # Arguments
/// * `a` - First number
/// * `b` - Second number
///
/// # Returns
/// The sum of a and b
///
/// # Examples
///
/// ```
/// assert_eq!(add(2, 2), 4);
/// assert_eq!(add(-1, 1), 0);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two floating-point numbers.
///
/// # Panics
/// Panics if either input is NaN
///
/// # Examples
///
/// ```
/// assert_eq!(multiply(2.0, 3.0), 6.0);
/// ```
pub fn multiply(a: f64, b: f64) -> f64 {
    assert!(!a.is_nan() && !b.is_nan());
    a * b
}
```

### Crate Documentation (`//!`)

Documents the enclosing item (usually a module or crate):

**src/lib.rs:**

```rust
//! A simple math library.
//!
//! Provides basic arithmetic operations.
//!
//! # Examples
//!
//! ```
//! use my_library::add;
//! assert_eq!(add(2, 3), 5);
//! ```

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

---

## Markdown in Documentation

Documentation supports full Markdown:

```rust
/// Formats a greeting message.
///
/// # Example
///
/// ```
/// let greeting = greet("Alice");
/// assert_eq!(greeting, "Hello, Alice!");
/// ```
///
/// # Panics
///
/// Panics if the name is empty.
///
/// # See also
///
/// * `greet_with_title` - Adds a title
/// * `format_greeting` - Lower-level function
pub fn greet(name: &str) -> String {
    if name.is_empty() {
        panic!("Name cannot be empty");
    }
    format!("Hello, {}!", name)
}
```

### Common Sections

- **# Examples** — Usage examples
- **# Panics** — When the function panics
- **# Errors** — When returning Result, what errors can occur
- **# Safety** — For unsafe functions, what invariants must be maintained
- **# Arguments** — Describe parameters (alternative to inline docs)
- **# Returns** — Describe return value
- **# See also** — Related functions

---

## Doc Tests

Code examples in doc comments are **executable tests**:

```rust
/// Adds two numbers.
///
/// # Examples
///
/// ```
/// assert_eq!(my_library::add(2, 2), 4);
/// assert_eq!(my_library::add(-1, 1), 0);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

**Run doc tests:**

```bash
cargo test --doc
```

**Output:**
```
running 1 test
test my_library::add::tests ... ok

test result: ok. 1 passed; 0 failed; 0 ignored
```

### Controlling Doc Tests

```rust
/// Example that should not compile:
///
/// ```ignore
/// my_library::add("string", 5);  // Type error
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Example that should panic:
///
/// ```should_panic
/// panic!("This panics");
/// ```
pub fn will_panic() {
    panic!("This panics");
}

/// Example that compiles but doesn't run:
///
/// ```no_run
/// // This would hang forever
/// loop {}
/// ```
pub fn infinite() {
    loop {}
}
```

---

## Generating Documentation

### Generate HTML Docs

```bash
# Generate and open in browser
cargo doc --open

# Generate without opening
cargo doc

# Include private items
cargo doc --open --document-private-items
```

**Output location**: `target/doc/your_crate/index.html`

### Publishing to docs.rs

When you publish to crates.io, documentation automatically appears on docs.rs:

```bash
cargo publish
```

Then visit: `https://docs.rs/your_crate/`

---

## Practical Examples

### Well-Documented Function

```rust
/// Finds the index of the maximum element in a slice.
///
/// Returns `None` if the slice is empty.
///
/// # Arguments
///
/// * `data` - A slice of comparable values
///
/// # Returns
///
/// * `Some(index)` - The index of the maximum element
/// * `None` - If the slice is empty
///
/// # Examples
///
/// ```
/// use my_library::max_index;
///
/// let data = vec![10, 5, 20, 15];
/// assert_eq!(max_index(&data), Some(2));
///
/// let empty: Vec<i32> = vec![];
/// assert_eq!(max_index(&empty), None);
/// ```
pub fn max_index(data: &[i32]) -> Option<usize> {
    if data.is_empty() {
        return None;
    }
    
    let mut max_val = data[0];
    let mut max_idx = 0;
    
    for (i, &val) in data.iter().enumerate() {
        if val > max_val {
            max_val = val;
            max_idx = i;
        }
    }
    
    Some(max_idx)
}
```

### Module Documentation

**src/math.rs:**

```rust
//! Mathematical utilities.
//!
//! This module provides basic mathematical functions.
//!
//! # Examples
//!
//! ```
//! use my_crate::math;
//!
//! let result = math::factorial(5);
//! assert_eq!(result, 120);
//! ```

/// Computes n factorial (n!).
///
/// # Examples
///
/// ```
/// use my_crate::math::factorial;
/// assert_eq!(factorial(5), 120);
/// assert_eq!(factorial(0), 1);
/// ```
pub fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}
```

**src/lib.rs:**

```rust
//! My Crate: A collection of utilities.
//!
//! This crate provides mathematical functions and string utilities.

pub mod math;

pub fn greet() {
    println!("Hello from my_crate!");
}
```

---

## Documentation Best Practices

### ✅ DO

- Document public APIs completely
- Include examples for non-trivial functions
- Explain why, not just what
- Document panics, errors, and edge cases
- Use examples that actually compile

### ❌ DON'T

- Leave documentation comments if they don't add value
- Copy function signatures as documentation
- Use examples that don't compile
- Document implementation details
- Forget to update docs when changing code

---

## Key Takeaways

✅ **Use `//` for comments** — regular comments explaining code.

✅ **Use `///` for public items** — document functions, structs, modules.

✅ **Use `//!` for crate/module documentation** — document the enclosing item.

✅ **Doc comments support Markdown** — format examples clearly.

✅ **Examples in docs are tests** — they run automatically with `cargo test --doc`.

✅ **Generate HTML docs** with `cargo doc --open`.

✅ **Document edge cases** — panics, errors, special behaviors.

✅ **Keep docs up-to-date** — stale docs are worse than no docs.

---

**Next:** [Ownership System](../03_ownership/ownership_rules.md) — Rust's most powerful feature.
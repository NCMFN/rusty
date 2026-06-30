# Variables and Mutability

## Declaring Variables

### Immutable Variables (Default)

In Rust, variables are **immutable by default**. Once you assign a value, you can't change it.

```rust
fn main() {
    let x = 5;
    println!("x is: {}", x);
    
    x = 6;  // ❌ ERROR: cannot assign twice to immutable variable
}
```

**Compiler error:**
```
error[E0384]: cannot assign twice to immutable variable `x`
  --> src/main.rs:5:5
   |
 3 |     let x = 5;
   |         - first assignment to `x`
 4 |     println!("x is: {}", x);
 5 |     x = 6;
   |     ^^^^^ cannot assign twice to immutable variable
```

This is intentional! Immutability prevents bugs.

### Mutable Variables

Use `mut` keyword to make a variable mutable:

```rust
fn main() {
    let mut x = 5;  // mut keyword
    println!("x is: {}", x);
    
    x = 6;  // ✅ OK: x is mutable
    println!("x is now: {}", x);
}
```

**Output:**
```
x is: 5
x is now: 6
```

### The Philosophy

Rust's default immutability:
- 📌 **Prevents accidental mutations** — you must be explicit about what can change
- 📌 **Documents intent** — readers see `mut` and know the variable changes
- 📌 **Enables compiler optimizations** — immutable bindings are easier to optimize
- 📌 **Thread-safe by default** — immutable data is automatically thread-safe

---

## Variable Shadowing

**Shadowing** means declaring a new variable with the same name, hiding the previous one:

```rust
fn main() {
    let x = 5;
    println!("x is: {}", x);  // 5
    
    let x = x + 1;  // Shadowing: new binding
    println!("x is: {}", x);  // 6
    
    let x = x * 2;  // Shadowing again
    println!("x is: {}", x);  // 12
}
```

**Output:**
```
x is: 5
x is: 6
x is: 12
```

### Shadowing vs. Mutation

Shadowing is **different** from mutation:

```rust
fn main() {
    // Mutation: same variable changes value
    let mut x = 5;
    x = 6;  // x changes in place
    
    // Shadowing: new variable created with same name
    let y = 5;
    let y = y + 1;  // New binding, old y is hidden
}
```

**Key difference:**
- **Mutation**: Variable's value changes, same memory location
- **Shadowing**: New variable created, can have different type!

### Shadowing with Type Change

```rust
fn main() {
    let spaces = "   ";  // &str
    let spaces = spaces.len();  // Shadowed as usize
    println!("Spaces: {}", spaces);  // 3 (number)
}
```

With mutation, this would fail:

```rust
fn main() {
    let mut spaces = "   ";  // &str
    spaces = spaces.len();  // ❌ ERROR: type mismatch
}
```

**Compiler error:**
```
error[E0308]: mismatched types
   |
 2 |     let mut spaces = "   ";
   |         -----------  ----- expected due to this
 3 |     spaces = spaces.len();
   |              ^^^^^^^^^^^^ expected `&str`, found `usize`
```

---

## Constants

Constants are **always immutable** and must have their type explicitly annotated:

```rust
const MAX_CONNECTIONS: u32 = 100;
const DB_URL: &str = "postgresql://localhost";

fn main() {
    println!("Max: {}", MAX_CONNECTIONS);
    println!("URL: {}", DB_URL);
}
```

### Constants vs. Variables

| Aspect | const | let |
|--------|-------|-----|
| **Mutability** | Always immutable | Immutable by default, can be `mut` |
| **Type annotation** | Required | Inferred (optional explicit) |
| **Scope** | Global or local | Local to block |
| **Initialization** | Must be constant expression | Any expression |
| **When evaluated** | Compile time | Runtime |

```rust
const SECONDS_PER_MINUTE: u32 = 60;  // ✅ OK: constant expression
// const X: u32 = std::env::var("TEST");  // ❌ ERROR: not constant

fn main() {
    let x = 5;
    // let y: i32;  // ❌ ERROR: not initialized
    let y: i32 = 5;  // ✅ OK
}
```

---

## Variable Scope

Variables are valid only within their **scope** (the block they're declared in):

```rust
fn main() {
    let x = 5;  // x is created here
    
    {
        let y = 10;  // y is created in this block
        println!("x: {}, y: {}", x, y);  // Both accessible
    }  // y goes out of scope, dropped here
    
    // println!("{}", y);  // ❌ ERROR: y not in scope
    println!("x: {}", x);  // ✅ OK: x still in scope
}  // x dropped here
```

**Output:**
```
x: 5, y: 10
x: 5
```

### Nested Scopes

```rust
fn main() {
    let x = 1;
    
    if true {
        let x = 2;  // Shadowing in nested scope
        println!("Inner: {}", x);  // 2
    }
    
    println!("Outer: {}", x);  // 1 (original x)
}
```

---

## Naming Conventions

Rust uses **snake_case** for variables and functions:

```rust
let my_variable = 5;
let user_age = 25;
let is_active = true;

fn calculate_total() {
    // ...
}

fn get_user_name() {
    // ...
}
```

**Constants use SCREAMING_SNAKE_CASE:**

```rust
const MAX_CONNECTIONS: u32 = 100;
const DEFAULT_TIMEOUT_SECS: u64 = 30;
```

**Types use PascalCase:**

```rust
struct UserProfile {}
enum Status {}
```

---

## Common Patterns

### Unused Variables

If you declare a variable but don't use it, Rust warns you:

```rust
fn main() {
    let x = 5;  // ⚠️ WARNING: unused variable
}
```

**Suppress warning with underscore:**

```rust
fn main() {
    let _x = 5;  // ✅ OK: explicitly unused
}
```

### Multiple Variable Declarations

```rust
fn main() {
    let (x, y) = (5, 10);  // Tuple destructuring
    println!("x: {}, y: {}", x, y);
    
    let mut a = 1;
    let mut b = 2;
    (a, b) = (b, a);  // Swap
    println!("a: {}, b: {}", a, b);
}
```

**Output:**
```
x: 5, y: 10
a: 2, b: 1
```

---

## Key Takeaways

✅ **Variables are immutable by default** — use `let` for immutable, `let mut` for mutable.

✅ **Immutability is intentional** — prevents bugs, documents intent, enables optimizations.

✅ **Shadowing** creates new variables with the same name (can change type).

✅ **Constants** are always immutable, must have explicit type, evaluated at compile time.

✅ **Scope** determines where variables are valid (blocks, functions, modules).

✅ **Use snake_case** for variables and functions; SCREAMING_SNAKE_CASE for constants.

✅ **Compiler helps** — warnings for unused variables, errors for invalid mutations.

---

**Next:** [Data Types](data_types.md) — Integers, floats, booleans, characters, and more.
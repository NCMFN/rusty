# What is Rust?

## A Brief History

**Rust** is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. It emerged from Mozilla Research around 2009 as an experimental language, with Graydon Hoare as its original creator. The first major release (version 1.0) came in May 2015, and today Rust is used in production by companies like:

- **Mozilla** (Servo browser engine, Firefox)
- **Amazon** (AWS, Firecracker VM)
- **Google** (Android, ChromeOS)
- **Microsoft** (Windows, Azure)
- **Cloudflare** (infrastructure)
- **Meta** (systems programming)
- **Discord** (services and tools)

### Why Now?

For decades, systems programmers faced a binary choice:

1. **C/C++**: Fast and powerful, but manual memory management = crashes, security vulnerabilities
2. **Java/Python**: Safe and productive, but slow and need a runtime

Rust's innovation: **You can have safety AND performance without sacrificing either.**

---

## Rust's Core Philosophy

Rust is built on three pillars:

### 1. **Safety**

Rust eliminates entire classes of bugs at compile time:
- **No buffer overflows** (bounds checking on arrays)
- **No use-after-free** (ownership system)
- **No data races** (thread safety guaranteed)
- **No null pointer dereferences** (Option/Result types)

**Example**: This C code segfaults:
```c
int arr[5] = {1, 2, 3, 4, 5};
printf("%d\n", arr[10]);  // Out of bounds! Undefined behavior.
```

In Rust, this won't even compile:
```rust
let arr = [1, 2, 3, 4, 5];
println!("{}", arr[10]);  // ❌ Compiler error: index out of bounds
```

### 2. **Performance**

Rust has **zero-cost abstractions**—you get high-level syntax without runtime overhead:
- No garbage collection
- No interpreter
- Direct memory management with compiler assistance
- LLVM backend for optimizations

**Benchmark**: Rust typically matches or beats C++ in performance:
```rust
// Rust code compiles to machine code just like C
fn sum_vec(v: &[i32]) -> i32 {
    v.iter().sum()  // Inlines to a tight loop
}
```

### 3. **Productivity**

Despite strict compile-time checks, Rust enables rapid development:
- Strong type system catches bugs early
- Excellent error messages guide you to solutions
- Rich standard library and ecosystem
- Fearless refactoring (compiler has your back)

---

## The Rust Mindset

Rust is **opinionated**. It enforces best practices:

### Ownership
Every value has a single owner. When the owner goes out of scope, the value is cleaned up. No garbage collector needed.

```rust
{
    let s = String::from("hello");  // s owns the string
    println!("{}", s);
}  // s goes out of scope, string is freed automatically
```

### Borrowing
You can lend references to data without transferring ownership:

```rust
fn print_length(s: &String) {  // Borrow s
    println!("Length: {}", s.len());
}  // s is still valid here

let s = String::from("hello");
print_length(&s);  // Lend a reference
println!("{}", s); // Still works!
```

### No Null Pointers
Rust uses `Option<T>` and `Result<T, E>` types instead of null/exceptions:

```rust
// Instead of: int* ptr = NULL;
let value: Option<i32> = None;  // Explicit, type-safe

// Must handle the None case:
match value {
    Some(n) => println!("Value: {}", n),
    None => println!("No value"),
}
```

---

## How Rust Works

### Compilation Process

```
Rust Source Code (.rs)
        ↓
   Parser (creates AST)
        ↓
  Type Checker & Borrow Checker
        ↓
      LLVM IR
        ↓
   Machine Code (executable)
```

The **borrow checker** runs before compilation finishes. If it detects memory safety violations, compilation fails—preventing entire categories of runtime bugs.

### Example: Borrow Checker in Action

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;      // Immutable borrow
    let r2 = &s;      // Another immutable borrow
    let r3 = &mut s;  // ❌ ERROR: Can't borrow as mutable while immutable borrows exist
}
```

This prevents data races at compile time!

---

## Rust vs. Other Languages

### Rust vs. C

| Aspect | C | Rust |
|--------|---|------|
| Memory safety | Manual | Automatic (compiler-enforced) |
| Performance | Excellent | Excellent (same) |
| Null safety | Not enforced | Enforced (Option type) |
| Concurrency | Hard (race conditions) | Built-in (thread-safe) |
| Learning curve | Steep | Steep (different reasons) |

**Verdict**: Rust is C with safety built in. Use Rust when you need C's performance but can't afford C's bugs.

### Rust vs. C++

| Aspect | C++ | Rust |
|--------|-----|------|
| Performance | Excellent | Excellent |
| Backward compatibility | Strong | Not a priority |
| Complexity | Very high | High (but simpler) |
| Build times | Slow | Slower (but predictable) |
| Memory safety | Manual/RAII | Enforced by compiler |
| Standard library | Small | Growing |

**Verdict**: Rust is like a simpler, safer C++ designed from the ground up for modern systems.

### Rust vs. Python

| Aspect | Python | Rust |
|--------|--------|------|
| Speed | Slow | Fast |
| Development | Fast | Medium |
| Memory usage | High (VM) | Low |
| Threading | Limited (GIL) | Full |
| Deployment | Requires runtime | Single binary |
| Learning | Beginner-friendly | Steeper initially |

**Verdict**: Python for rapid prototyping; Rust for performance-critical production code.

### Rust vs. Go

| Aspect | Go | Rust |
|--------|---|------|
| Simplicity | Very simple | Learning curve |
| Concurrency | Built-in (goroutines) | Powerful (async/await) |
| Speed | Very fast | Very fast |
| Deployment | Single binary | Single binary |
| Type safety | Good | Excellent |
| Generics | Limited | Full |

**Verdict**: Go for rapid service development; Rust for maximum efficiency and type safety.

---

## When to Use Rust

✅ **Perfect for:**
- Systems programming (OS, kernels, drivers)
- Embedded systems
- High-performance applications (games, servers)
- Networking services
- CLI tools
- WebAssembly (WASM)
- Data processing pipelines
- Anything memory-critical

❌ **Not ideal for:**
- Rapid prototypes (compile times)
- One-off scripts (Python is faster)
- Heavy dynamic typing (though Rust has trait objects)
- Projects requiring only "good enough" performance

---

## What Makes Rust Different

### 1. No Runtime
No garbage collector, no VM. Pure machine code.

### 2. Fearless Concurrency
Thread safety is enforced at compile time. Data races are impossible.

### 3. Powerful Type System
Generics, traits, and pattern matching enable safe, high-level abstractions.

### 4. Excellent Error Messages
```rust
fn main() {
    let x = 5;
    x = 6;  // ❌
}
```

**Compiler output:**
```
error[E0384]: cannot assign twice to immutable variable `x`
  --> main.rs:3:5
   |
 2 |     let x = 5;
   |         - first assignment to `x`
 3 |     x = 6;
   |     ^^^^^ cannot assign twice to immutable variable

help: consider making this binding mutable
   |
 2 |     let mut x = 5;
   |         ^^^^

For more information about this error, try `rustc --explain E0384`.
```

### 5. REPL-less (Sort of)
No interactive REPL like Python, but `cargo` and tools make iteration fast.

---

## The Rust Ecosystem

Rust has a thriving ecosystem:

### Package Manager: Cargo
Manage dependencies, build, test, publish with one tool.

### Registry: crates.io
100,000+ community-maintained crates (libraries).

### Standard Library
Core data structures and utilities (Vec, HashMap, etc.)

### Popular Crates
- **serde**: Serialization/deserialization
- **tokio**: Async runtime
- **clap**: CLI argument parsing
- **reqwest**: HTTP client
- **sqlx**: Database access
- **diesel**: ORM

---

## A Complete First Program

```rust
use std::io;

fn main() {
    println!("Guess the number!");
    
    let secret = 42;
    let mut tries = 0;
    
    loop {
        println!("\nEnter your guess:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        
        let guess: i32 = input.trim().parse()
            .expect("Please enter a number");
        
        tries += 1;
        
        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("Too low!"),
            std::cmp::Ordering::Greater => println!("Too high!"),
            std::cmp::Ordering::Equal => {
                println!("Correct! You won in {} tries!", tries);
                break;
            }
        }
    }
}
```

This program demonstrates:
- Pattern matching (`match`)
- Mutable variables (`mut`)
- Error handling (`.expect()`)
- String parsing
- Looping

---

## Key Takeaways

✅ **Rust is a systems language** that combines C's performance with modern safety guarantees.

✅ **The borrow checker** prevents entire categories of bugs (data races, use-after-free, buffer overflows) at compile time.

✅ **No garbage collection** means predictable performance and simple deployment (single binary).

✅ **Strong type system** catches bugs early and enables powerful abstractions.

✅ **Rust is opinionated** about correctness—it feels strict at first, but becomes liberating once you understand the philosophy.

✅ **The community is welcoming** and the tooling (cargo, clippy) is excellent.

✅ **Rust is practical** for real-world use: systems, web services, CLI tools, games, and more.

---

**Next:** [Why Rust?](why_rust.md) — A deeper dive into the problems Rust solves.
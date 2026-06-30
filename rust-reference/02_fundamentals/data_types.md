# Data Types

## Type System Overview

Rust is **statically typed** — every value has a type known at compile time. However, Rust usually **infers types** for you.

```rust
fn main() {
    let x = 5;              // Inferred as i32
    let y = 5.0;            // Inferred as f64
    let z: i64 = 5;         // Explicitly i64
    let b: bool = true;     // Explicitly bool
}
```

---

## Scalar Types

A **scalar** is a single value. Rust has four scalar types:

### 1. Integer Types

**Signed integers** (can be negative):

| Type | Size | Range |
|------|------|-------|
| `i8` | 1 byte | -128 to 127 |
| `i16` | 2 bytes | -32,768 to 32,767 |
| `i32` | 4 bytes | -2,147,483,648 to 2,147,483,647 |
| `i64` | 8 bytes | ±9 quintillion |
| `i128` | 16 bytes | ±170 undecillion |
| `isize` | Platform | Pointer-sized |

**Unsigned integers** (only positive):

| Type | Size | Range |
|------|------|-------|
| `u8` | 1 byte | 0 to 255 |
| `u16` | 2 bytes | 0 to 65,535 |
| `u32` | 4 bytes | 0 to 4,294,967,295 |
| `u64` | 8 bytes | 0 to 18 quintillion |
| `u128` | 16 bytes | 0 to 340 undecillion |
| `usize` | Platform | Pointer-sized |

**Default**: When type is ambiguous, integers default to `i32`.

```rust
fn main() {
    let x = 5;           // i32 (default)
    let y: u8 = 255;     // u8 (explicit)
    let z: i64 = -100;   // i64 (explicit)
    
    // Number separators for readability
    let million = 1_000_000;
    let hex = 0xFF;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';     // u8
}
```

**Integer overflow:**

```rust
fn main() {
    let x: u8 = 255;
    let y = x + 1;  // ⚠️ Panic in debug mode, wraps in release
}
```

In debug builds, overflow panics. In release builds, it wraps (255 + 1 = 0). Use `checked_add` for safe math:

```rust
fn main() {
    let x: u8 = 255;
    match x.checked_add(1) {
        Some(result) => println!("Result: {}", result),
        None => println!("Overflow!"),
    }
}
```

### 2. Floating-Point Types

Rust supports IEEE 754 floating-point numbers:

```rust
fn main() {
    let x = 2.0;        // f64 (default)
    let y: f32 = 3.14;  // f32 (explicit)
    
    // Special values
    let inf = f64::INFINITY;
    let neg_inf = f64::NEG_INFINITY;
    let nan = f64::NAN;
    
    // Math operations
    let z = x + y as f64;  // Type cast needed
}
```

**Beware of floating-point precision:**

```rust
fn main() {
    let x = 0.1 + 0.2;
    println!("x = {}", x);  // 0.30000000000000004 (not exactly 0.3)
    
    // Don't use == for floats
    if (x - 0.3).abs() < 1e-10 {
        println!("Close enough!");
    }
}
```

### 3. Boolean Type

```rust
fn main() {
    let t = true;
    let f: bool = false;
    
    // Booleans in conditions
    if t {
        println!("true branch");
    }
    
    // Logical operations
    let result = true && false;  // AND: false
    let result = true || false;  // OR: true
    let result = !true;          // NOT: false
}
```

### 4. Character Type

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ';         // Unicode
    let heart_eyed_cat = '😻';  // Emoji
    
    // char is 4 bytes (not 1 byte like in C)
    println!("Size: {} bytes", std::mem::size_of_val(&c));
}
```

**Output:**
```
Size: 4 bytes
```

Characters are Unicode scalars, not ASCII.

---

## Compound Types

**Compound types** group multiple values into one type.

### 1. Tuple

Fixed-length collection of values with **different types**:

```rust
fn main() {
    // Declaration
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // Destructuring
    let (x, y, z) = tup;
    println!("y = {}", y);
    
    // Indexing
    let x = tup.0;  // 500
    let y = tup.1;  // 6.4
    let z = tup.2;  // 1
}
```

**Unit tuple (empty):**

```rust
fn main() {
    let unit = ();
    println!("Unit: {:?}", unit);  // ()
}
```

### 2. Array

Fixed-length collection of values with the **same type**:

```rust
fn main() {
    // Declaration
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Shorthand (all same value)
    let arr = [3; 5];  // [3, 3, 3, 3, 3]
    
    // Indexing
    let first = arr[0];
    let second = arr[1];
    
    // Iteration
    for element in &arr {
        println!("Element: {}", element);
    }
}
```

**Bounds checking:**

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let index = 10;
    let element = arr[index];  // ❌ PANIC: index out of bounds
}
```

Rust panics on invalid array access (compile-time check when index is known):

```bash
$ cargo run
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10'
```

---

## Type Casting

Convert between numeric types with `as`:

```rust
fn main() {
    let x: i32 = 5;
    let y = x as f64;      // i32 → f64
    let z = y as i32;      // f64 → i32
    
    let ch: u8 = 65;
    let c = ch as char;    // u8 → char ('A')
    let num = c as u32;    // char → u32 (65)
    
    // Truncates if narrowing
    let x: f32 = 3.7;
    let y = x as u8;       // 3 (truncated)
}
```

**Be careful with casting:**

```rust
fn main() {
    // Unsigned to signed
    let x: u32 = 300;
    let y = x as i8;  // -12 (wraps around)
    
    // Float to int
    let f = 3.9_f64;
    let i = f as i32;  // 3 (truncates, not rounds)
}
```

---

## Type Inference Examples

```rust
fn main() {
    // Inferred from literal
    let x = 5;              // i32
    let y = 5.0;            // f64
    let c = 'a';            // char
    let b = true;           // bool
    
    // Inferred from operation
    let x = 5 + 6;          // i32
    let x = 5.0 + 6.0;      // f64
    let x = [1, 2, 3];      // [i32; 3]
    
    // Inferred from method call
    let s = "hello".len();  // usize
    
    // Inferred from return type
    let x: i32 = "5".parse().unwrap();  // Must specify type
}
```

**When parsing strings, you must specify the type:**

```rust
fn main() {
    // ❌ ERROR: type can't be inferred
    let x = "42".parse().unwrap();
    
    // ✅ OK: type is specified
    let x: i32 = "42".parse().unwrap();
    let y: f64 = "3.14".parse().unwrap();
}
```

---

## Common Patterns

### Printing Type Names at Runtime

```rust
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let x = 5;
    print_type_of(&x);  // i32
    
    let y = 5.0;
    print_type_of(&y);  // f64
}
```

### Checking Type at Compile Time

```rust
fn main() {
    let x = 5;
    // This won't compile if x is not i32
    let _: i32 = x;
}
```

---

## Key Takeaways

✅ **Rust is statically typed** but uses type inference to avoid boilerplate.

✅ **Scalar types**: i32/u32 (integers), f64 (floats), bool, char.

✅ **Default numeric type**: `i32` for integers, `f64` for floats.

✅ **Compound types**: Tuples (different types), Arrays (same type, fixed length).

✅ **Type casting** with `as` keyword is explicit.

✅ **Integer overflow** panics in debug, wraps in release.

✅ **Floating-point precision** issues exist; use `.abs()` for comparisons.

✅ **char is 4 bytes** (Unicode), not 1 byte like in C.

---

**Next:** [Functions](functions.md) — Declare and use functions effectively.
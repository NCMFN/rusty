# Ownership Rules

## The Three Rules of Ownership

Rust's ownership system is based on three fundamental rules:

### Rule 1: Each value has exactly one owner

Every piece of data in memory has a single owner responsible for it.

```rust
fn main() {
    let s = String::from("hello");  // s owns the String
    println!("{}", s);              // s is the owner
}  // s goes out of scope, the String is dropped
```

### Rule 2: You can transfer ownership (move)

When you assign a value to another variable, ownership transfers:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // Ownership moves to s2
    
    println!("{}", s1);  // ❌ ERROR: s1 no longer owns the data
    println!("{}", s2);  // ✅ OK: s2 owns it
}
```

**Compiler error:**
```
error[E0382]: borrow of moved value: `s1`
  --> src/main.rs:5:20
   |
3 |     let s1 = String::from("hello");
4 |     let s2 = s1;
   |              -- value moved here
5 |     println!("{}", s1);
   |                    ^^ value borrowed after move
```

### Rule 3: When the owner goes out of scope, the value is dropped

Memory is automatically freed. No garbage collector needed.

```rust
fn main() {
    {
        let s = String::from("hello");
        println!("{}", s);
    }  // s goes out of scope, memory is freed here
    
    // println!("{}", s);  // ❌ s is no longer valid
}
```

---

## Move Semantics

**Move** means transferring ownership from one variable to another.

### Moves with Variables

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // MOVE: s1 moves to s2
    
    // s1 is now invalid, s2 owns the data
    println!("s2: {}", s2);  // ✅ hello
}
```

### Moves with Functions

**Passing to a function:**

```rust
fn takes_ownership(s: String) {
    println!("{}", s);
}  // s goes out of scope, the String is dropped

fn main() {
    let s = String::from("hello");
    takes_ownership(s);  // MOVE: s moves into the function
    
    // println!("{}", s);  // ❌ ERROR: s no longer valid
}
```

**Returning from a function:**

```rust
fn gives_ownership() -> String {
    String::from("hello")
}  // The String moves out of the function

fn main() {
    let s = gives_ownership();  // MOVE: function returns, s takes ownership
    println!("{}", s);          // ✅ hello
}
```

**Taking and giving back:**

```rust
fn takes_and_gives_back(s: String) -> String {
    s  // s moves out of the function
}

fn main() {
    let s1 = String::from("hello");
    let s2 = takes_and_gives_back(s1);  // s1 moves in, s2 takes ownership
    
    // println!("{}", s1);  // ❌ s1 no longer valid
    println!("{}", s2);     // ✅ hello
}
```

---

## Copy Types

Some types **don't** move — they're **copied** instead.

### Types that Implement Copy

All integers, floats, booleans, and characters are `Copy`:

```rust
fn main() {
    let x = 5;
    let y = x;  // COPY: x is copied to y, x still valid
    
    println!("x: {}, y: {}", x, y);  // ✅ Both valid
}
```

**Why?** These types are small and simple. Copying is cheap.

### Copy vs. Move

| Type | Behavior | Size |
|------|----------|------|
| `i32`, `f64`, `bool`, `char` | Copy | Small (≤128 bits) |
| `String` | Move | Large (heap-allocated) |
| `Vec<T>` | Move | Large (heap-allocated) |
| `Box<T>` | Move | Pointer to heap |

```rust
fn main() {
    // Copy types
    let x = 5;
    let y = x;  // x is still valid ✅
    
    // Move types
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is no longer valid ❌
}
```

### The Copy Trait

Types that implement `Copy` are automatically copied instead of moved. You can't implement `Copy` for types with heap-allocated data:

```rust
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,  // Only small types
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;  // COPY: both are valid
    println!("p1: ({}, {})", p1.x, p1.y);  // ✅
}
```

---

## Practical Examples

### Building a String

```rust
fn build_message(name: String) -> String {
    let greeting = String::from("Hello, ");
    let mut result = greeting;  // MOVE
    result.push_str(&name);     // Borrow name
    result                       // MOVE out
}

fn main() {
    let name = String::from("Alice");
    let message = build_message(name);  // MOVE
    
    println!("{}", message);  // ✅ Hello, Alice
    // println!("{}", name);  // ❌ name was moved
}
```

### Swapping Values

```rust
fn swap(a: String, b: String) -> (String, String) {
    (b, a)  // Return swapped (moved out)
}

fn main() {
    let s1 = String::from("first");
    let s2 = String::from("second");
    
    let (s1, s2) = swap(s1, s2);  // Move in, move out
    
    println!("s1: {}, s2: {}", s1, s2);
    // s1: second, s2: first
}
```

---

## Common Mistakes

### Use After Move

```rust
fn main() {
    let s = String::from("hello");
    println!("{}", s);  // ✅
    let t = s;          // MOVE
    println!("{}", s);  // ❌ ERROR: s was moved
}
```

**Fix**: Use a reference (borrow) instead of moving:

```rust
fn main() {
    let s = String::from("hello");
    println!("{}", &s);  // ✅ Borrow
    let t = &s;          // ✅ Borrow
    println!("{}", &s);  // ✅ Still valid
}
```

### Unexpected Moves

```rust
fn takes_ownership(s: String) {}

fn main() {
    let s = String::from("hello");
    takes_ownership(s);  // MOVE
    println!("{}", s);  // ❌ ERROR: s was moved into function
}
```

**Fix**: Pass a reference:

```rust
fn takes_ownership(s: &String) {}  // Borrow, don't take ownership

fn main() {
    let s = String::from("hello");
    takes_ownership(&s);  // ✅ Borrow
    println!("{}", s);    // ✅ Still valid
}
```

---

## Memory Model

### Stack vs. Heap

**Stack**: Fixed-size data (integers, booleans, small structs)
- Push/pop operations
- Very fast
- Automatically cleaned up

**Heap**: Variable-size data (Strings, Vectors, large objects)
- Allocate/deallocate with pointers
- Slower
- Requires manual management (Rust does this for you)

```rust
fn main() {
    let x = 5;                      // Stack: integer
    let s = String::from("hello");  // Heap: String with pointer on stack
    
    // Stack layout:
    // x: 5
    // s: {ptr: 0x1000, len: 5, capacity: 5}
    //
    // Heap layout (at 0x1000):
    // "hello" (5 bytes)
}
```

### Move in Memory

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // MOVE: copy the pointer
    
    // Stack:
    // s1: {ptr: null}      ← invalidated
    // s2: {ptr: 0x1000, ...} ← now owns
    //
    // Heap (at 0x1000): "hello"
}
```

Only the **pointer, length, capacity** move. The actual heap data stays put!

---

## Key Takeaways

✅ **Each value has one owner** — prevents multiple freed pointers.

✅ **Ownership can transfer (move)** — when assigned or passed to functions.

✅ **Copy types are copied** — integers, floats, booleans, chars.

✅ **Move types are moved** — Strings, Vecs, Boxes.

✅ **Drop trait** — when owner goes out of scope, memory is freed.

✅ **No garbage collector needed** — ownership is deterministic.

✅ **Compile-time safety** — use after move is caught before runtime.

✅ **Use borrowing** to avoid unwanted moves (next section).

---

**Next:** [Borrowing](borrowing.md) — Reference without transferring ownership.
# Borrowing and the Borrow Checker

## What is Borrowing?

**Borrowing** lets you use a value without taking ownership. You create a **reference** to the data:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = &s1;  // Borrow s1 (create a reference)
    
    println!("s1: {}", s1);  // ✅ s1 still owns it
    println!("s2: {}", s2);  // ✅ s2 borrows it
}  // Both go out of scope, but only s1 drops the data
```

**Output:**
```
s1: hello
s2: hello
```

---

## Immutable References (&T)

### Creating Immutable References

Use `&` to create an immutable (shared) reference:

```rust
fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

fn main() {
    let s = String::from("hello");
    print_length(&s);  // Pass a reference
    println!("s: {}", s);  // s still valid
}
```

### Multiple Immutable References

You can have **many immutable references** to the same data:

```rust
fn main() {
    let s = String::from("hello");
    
    let r1 = &s;  // Immutable borrow
    let r2 = &s;  // Another immutable borrow
    let r3 = &s;  // And another
    
    println!("r1: {}", r1);  // ✅
    println!("r2: {}", r2);  // ✅
    println!("r3: {}", r3);  // ✅
    println!("s: {}", s);    // ✅ Original still valid
}
```

**Why?** Multiple readers can't corrupt data.

---

## Mutable References (&mut T)

### Creating Mutable References

Use `&mut` to create a mutable (exclusive) reference:

```rust
fn change_string(s: &mut String) {
    s.push_str(" world");
}

fn main() {
    let mut s = String::from("hello");
    change_string(&mut s);  // Pass mutable reference
    println!("s: {}", s);   // hello world
}
```

**Key requirement**: The variable must be declared `mut`.

### Only One Mutable Reference at a Time

You can have **only one mutable reference** to a piece of data at a time:

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &mut s;
    let r2 = &mut s;  // ❌ ERROR: can't have two mutable borrows
    
    println!("{}, {}", r1, r2);
}
```

**Compiler error:**
```
error[E0499]: cannot borrow `s` as mutable more than once at a time
  --> src/main.rs:5:14
   |
4 |     let r1 = &mut s;
   |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
   |              ^^^^^^ second mutable borrow occurs here
6 |     
7 |     println!("{}, {}", r1, r2);
   |                       -- first borrow later used here
```

**Why?** Only one writer prevents data races.

---

## The Borrow Checker Rules

At any given time, you can have **either**:
- Any number of **immutable references** (&T), OR
- Exactly one **mutable reference** (&mut T)

But **never both at the same time**.

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;      // Immutable borrow ✅
    let r2 = &s;      // Immutable borrow ✅
    let r3 = &mut s;  // ❌ ERROR: can't mutable borrow while immutable borrows exist
    
    println!("{}", r1);
}
```

**Compiler error:**
```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
```

### Scope Matters

Borrows are only active while they're used:

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);  // r1, r2 used here
    
    let r3 = &mut s;  // ✅ OK: r1 and r2 are no longer needed
    r3.push_str(" world");
    println!("{}", r3);
}
```

**Output:**
```
hello, hello
hello world
```

The compiler tracks where each reference is **last used** (Non-Lexical Lifetimes, or NLL).

---

## Dereferencing

Dereference a reference with `*` to access the value:

```rust
fn main() {
    let s = String::from("hello");
    let r = &s;
    
    // Dereference with *
    println!("Length: {}", (*r).len());
    
    // But usually you don't need to dereference explicitly
    println!("Length: {}", r.len());  // Auto-deref in method calls
}
```

**Auto-deref** happens in method calls, so you don't usually need `*`:

```rust
fn main() {
    let s = String::from("hello");
    let r = &s;
    
    // These are equivalent
    println!("{}", (*r).len());  // Explicit deref
    println!("{}", r.len());     // Auto-deref (preferred)
}
```

---

## Preventing Data Races at Compile Time

### Example: Data Race (Not Possible in Rust)

In C++ or Java, this would be a data race:

```c
// C++ - RACE CONDITION!
int x = 0;
std::thread t1([&x] { x = 1; });  // Thread 1: write
std::thread t2([&x] { x = 2; });  // Thread 2: write
// Result: undefined behavior
```

In Rust, this is **impossible**:

```rust
use std::thread;

fn main() {
    let mut x = 0;
    
    let t1 = thread::spawn(|| {
        x = 1;  // ❌ ERROR: can't write while other thread might access
    });
    
    let t2 = thread::spawn(|| {
        x = 2;  // ❌ ERROR: same issue
    });
}
```

**Compiler error**: Can't move mutable reference into two threads.

**Safe equivalent** (using `Arc<Mutex<T>>`):

```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let x = Arc::new(Mutex::new(0));
    
    let x1 = Arc::clone(&x);
    let t1 = thread::spawn(move || {
        let mut guard = x1.lock().unwrap();
        *guard = 1;
    });
    
    let x2 = Arc::clone(&x);
    let t2 = thread::spawn(move || {
        let mut guard = x2.lock().unwrap();
        *guard = 2;
    });
    
    t1.join().unwrap();
    t2.join().unwrap();
}
```

---

## Practical Examples

### Mutating Through References

```rust
fn append_greeting(s: &mut String, name: &str) {
    s.push_str(", ");
    s.push_str(name);
}

fn main() {
    let mut greeting = String::from("Hello");
    let name = "Alice";
    
    append_greeting(&mut greeting, name);
    println!("{}", greeting);  // Hello, Alice
}
```

### Borrowing in Structs

```rust
struct Person {
    name: String,
    age: u32,
}

fn print_person(p: &Person) {
    println!("Name: {}, Age: {}", p.name, p.age);
}

fn age_person(p: &mut Person) {
    p.age += 1;
}

fn main() {
    let mut alice = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    print_person(&alice);  // ✅ Borrow
    age_person(&mut alice); // ✅ Mutable borrow
    print_person(&alice);   // ✅ Borrow again
}
```

---

## Common Mistakes

### Can't Modify Through Immutable Reference

```rust
fn main() {
    let s = String::from("hello");
    let r = &s;
    r.push_str(" world");  // ❌ ERROR: can't mutate through immutable ref
}
```

**Compiler error:**
```
error[E0596]: cannot borrow `*r` as mutable, as it is behind a shared ("immutable") reference
```

**Fix**: Use `&mut`:

```rust
fn main() {
    let mut s = String::from("hello");
    let r = &mut s;
    r.push_str(" world");  // ✅ OK
}
```

### Mixing Immutable and Mutable Borrows

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;      // Immutable borrow
    let r2 = &mut s;  // ❌ ERROR: can't mutable borrow while immutable borrows exist
    
    println!("{}", r1);
}
```

**Fix**: Let immutable borrows end before creating mutable borrow:

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;
    println!("{}", r1);  // r1 no longer needed
    
    let r2 = &mut s;  // ✅ OK: r1's scope ended
    r2.push_str(" world");
}
```

---

## Key Takeaways

✅ **Borrowing avoids moves** — use `&T` to lend data.

✅ **Immutable references (`&T`)** — multiple readers, no mutations.

✅ **Mutable references (`&mut T`)** — exclusive writer, can mutate.

✅ **Borrow checker enforces** — no data races, no use-after-free.

✅ **Never have both** immutable and mutable references to the same data simultaneously.

✅ **Scope matters** — borrows end when last used (NLL).

✅ **Auto-deref** — method calls automatically dereference.

✅ **Compile-time safety** — data races are impossible in Rust.

---

**Next:** [Slices](slices.md) — References to parts of collections.
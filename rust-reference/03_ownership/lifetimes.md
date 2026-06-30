# Lifetimes

## What Are Lifetimes?

**Lifetimes** are annotations that specify how long references are valid. They ensure references don't outlive the data they point to.

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;  // ❌ ERROR: x won't live long enough
    }
    println!("{}", r);  // r points to dropped data
}
```

**Compiler error:**
```
error[E0597]: `x` does not live long enough
  --> src/main.rs:5:13
   |
4  |         r = &x;
   |             -- borrow occurs here
5  |     }
   |     - `x` dropped here while still borrowed
6  |     println!("{}", r);
   |                     - borrow later used here
```

---

## Lifetime Annotations

Use `'` to name lifetimes (pronounced "tick"):

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

**Compiler error:**
```
error: missing lifetime specifier
  --> src/main.rs:1:33
   |
1  | fn longest(x: &str, y: &str) -> &str {
   |               ----  ----      ^
   |               |
   |               expected named lifetime parameter
```

The compiler can't tell which input reference the output refers to.

### Adding Lifetime Annotations

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("long string");
    let s2 = String::from("xyz");
    let result = longest(&s1, &s2);
    println!("Longest: {}", result);
}
```

**Output:**
```
Longest: long string
```

### What Lifetimes Mean

`'a` is a **generic lifetime parameter**. It represents "some lifetime."

**In `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`:**
- `x` has lifetime `'a`
- `y` has lifetime `'a`
- The return value also has lifetime `'a`

This means: **The returned reference will be valid as long as both input references are valid.**

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("long string");
    {
        let s2 = String::from("xyz");
        let result = longest(&s1, &s2);
        println!("{}", result);  // ✅ Both s1 and s2 still valid
    }
    // println!("{}", result);  // ❌ s2 is dropped
}
```

---

## Lifetime Elision

Rust uses **lifetime elision rules** to infer lifetimes in common cases. You don't always need explicit annotations:

### Rule 1: Each Parameter Gets Its Own Lifetime

```rust
// Without elision (explicit)
fn print_ref<'a>(x: &'a str) {}

// With elision (inferred)
fn print_ref(x: &str) {}  // Same meaning
```

### Rule 2: If There's Only One Input Lifetime, It's the Output

```rust
// Without elision (explicit)
fn first_word<'a>(s: &'a str) -> &'a str { &s[..] }

// With elision (inferred)
fn first_word(s: &str) -> &str { &s[..] }  // Same meaning
```

### Rule 3: If There's `&self` or `&mut self`, Its Lifetime is the Output

```rust
struct Person {
    name: String,
}

impl Person {
    // Without elision (explicit)
    fn name<'a>(&'a self) -> &'a str { &self.name }
    
    // With elision (inferred)
    fn name(&self) -> &str { &self.name }  // Same meaning
}
```

### When Elision Fails

When you have multiple input references and the compiler can't infer which one the output refers to:

```rust
fn longest(x: &str, y: &str) -> &str {  // ❌ ERROR
    if x.len() > y.len() { x } else { y }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {  // ✅ OK
    if x.len() > y.len() { x } else { y }
}
```

---

## Lifetime Examples

### Struct with References

When a struct holds a reference, it needs a lifetime:

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael.");
    let first_sentence = &novel[0..16];
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("Excerpt: {}", excerpt.part);
}
```

**Why?** The struct holds a reference to `novel`. It can't outlive `novel`:

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let excerpt;
    {
        let novel = String::from("Call me Ishmael.");
        excerpt = ImportantExcerpt {
            part: &novel,  // ❌ ERROR: novel dropped
        };
    }
    println!("{}", excerpt.part);  // novel no longer exists
}
```

### Methods with Lifetimes

```rust
struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
    
    fn introduce(&self, other: &'a str) -> String {
        format!("{} meets {}", self.name, other)
    }
}

fn main() {
    let name = "Alice";
    let alice = Person { name };
    
    println!("{}", alice.greet());
    println!("{}", alice.introduce("Bob"));
}
```

---

## The 'static Lifetime

`'static` means the reference lives for the **entire program**:

```rust
fn main() {
    let s: &'static str = "hello";  // String literals are 'static
    println!("{}", s);
}
```

### String Literals vs. Strings

```rust
fn main() {
    let literal: &'static str = "hello";  // ✅ String literal: 'static
    let s: String = String::from("hello");
    let s_ref: &str = &s;                  // ❌ Not 'static (tied to s)
}
```

### Generic Functions with 'static

```rust
fn print_it(s: &'static str) {
    println!("{}", s);
}

fn main() {
    print_it("hello");  // ✅ String literal
    
    let s = String::from("world");
    print_it(&s);  // ❌ ERROR: s is not 'static
}
```

---

## Multiple Lifetimes

You can use multiple lifetimes when they're independent:

```rust
fn announce<'a, 'b>(x: &'a str, y: &'b str) {
    println!("x: {}, y: {}", x, y);
}

fn main() {
    let x = String::from("hello");
    let y = String::from("world");
    announce(&x, &y);
}
```

**When are two lifetimes independent?** When they don't need to be related:

```rust
// Lifetimes are independent
fn announce<'a, 'b>(x: &'a str, y: &'b str) -> String {
    format!("{} {}", x, y)  // Returns owned String (no lifetime)
}

// Lifetimes are related
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }  // Returns reference
}
```

---

## Common Patterns

### Returning References from Owned Data

❌ **Impossible** — can't return reference to local data:

```rust
fn dangle() -> &String {  // ❌ ERROR
    let s = String::from("hello");
    &s  // s is dropped, reference is invalid
}
```

**Fix**: Return owned data:

```rust
fn no_dangle() -> String {  // ✅ OK
    let s = String::from("hello");
    s  // Returns owned data
}
```

### Borrowing from Function Parameter

✅ **Possible** — parameter outlives the function:

```rust
fn first_char(s: &str) -> &str {  // ✅ OK
    &s[..1]
}

fn main() {
    let s = String::from("hello");
    let first = first_char(&s);
    println!("{}", first);  // ✅ s still valid
}
```

---

## Debugging Lifetime Errors

### Error: "Borrow Does Not Live Long Enough"

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;  // ❌ x's lifetime is shorter than r's
    }
    println!("{}", r);
}
```

**Fix**: Ensure the borrowed data lives long enough:

```rust
fn main() {
    let x = 5;
    let r = &x;  // ✅ x lives as long as r
    println!("{}", r);
}
```

### Error: "Missing Lifetime Specifier"

```rust
fn longest(x: &str, y: &str) -> &str {  // ❌ Compiler can't infer
    if x.len() > y.len() { x } else { y }
}
```

**Fix**: Add lifetime annotations:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {  // ✅ OK
    if x.len() > y.len() { x } else { y }
}
```

---

## Key Takeaways

✅ **Lifetimes ensure references stay valid** — prevent dangling pointers.

✅ **Lifetime annotations use `'name`** — e.g., `&'a str`.

✅ **Lifetime elision** — Rust infers in common cases.

✅ **When multiple inputs**, you often need explicit lifetimes.

✅ **Structs with references** need lifetime parameters.

✅ **`'static` lifetime** — lives for entire program (string literals).

✅ **Compiler guides you** — error messages explain what's missing.

✅ **Lifetimes are compile-time only** — zero runtime cost.

---

**Next:** [Structs and Enums](../04_structs_and_enums/structs.md) — Creating custom types.
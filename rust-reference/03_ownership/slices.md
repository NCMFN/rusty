# Slices

## What are Slices?

A **slice** is a reference to a contiguous sequence of elements in a collection. It's also called a **borrowed collection**.

```rust
fn main() {
    let s = String::from("hello world");
    let slice = &s[0..5];  // "hello"
    println!("{}", slice);
}
```

**Output:**
```
hello
```

Slices let you reference part of a collection without owning it.

---

## String Slices (&str)

### Creating String Slices

```rust
fn main() {
    let s = String::from("hello world");
    
    // Slice indices: [start..end] (end is exclusive)
    let hello = &s[0..5];   // "hello"
    let world = &s[6..11];  // "world"
    
    println!("hello: {}", hello);
    println!("world: {}", world);
}
```

**Output:**
```
hello: hello
world: world
```

### Slice with Omitted Boundaries

```rust
fn main() {
    let s = String::from("hello world");
    
    let hello = &s[..5];     // From start to index 5
    let world = &s[6..];     // From index 6 to end
    let all = &s[..];        // The entire string
    
    println!("hello: {}", hello);  // hello
    println!("world: {}", world);  // world
    println!("all: {}", all);      // hello world
}
```

### String Slice Type: &str

`&str` is a **string slice** — a reference to UTF-8 text:

```rust
fn main() {
    let s = String::from("hello world");
    let slice: &str = &s[0..5];  // String slice
    
    // String literals are also &str
    let literal: &str = "hello";
    
    println!("Slice: {}", slice);    // hello
    println!("Literal: {}", literal); // hello
}
```

### Practical Example: Finding Word Boundaries

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {  // b' ' is the space character
            return &s[0..i];
        }
    }
    
    &s[..]  // Return entire string if no space
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("First word: {}", word);  // hello
    
    let literal = "hello";
    let word = first_word(literal);
    println!("First word: {}", word);  // hello
}
```

**Output:**
```
First word: hello
First word: hello
```

**Key insight**: `first_word` accepts both `&String` and `&str` because it takes `&str`.

---

## Array Slices (&[T])

### Creating Array Slices

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    
    let slice = &arr[1..4];  // [2, 3, 4]
    println!("Slice: {:?}", slice);
    
    for &element in slice {
        println!("Element: {}", element);
    }
}
```

**Output:**
```
Slice: [2, 3, 4]
Element: 2
Element: 3
Element: 4
```

### Vector Slices

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    let slice = &v[1..3];  // [2, 3]
    println!("Slice: {:?}", slice);
}
```

### Slice Type: &[T]

The type `&[T]` represents a slice of type `T`:

```rust
fn print_slice(slice: &[i32]) {
    println!("Slice: {:?}", slice);
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let v = vec![1, 2, 3, 4, 5];
    
    print_slice(&arr[..]);    // ✅ Array slice
    print_slice(&v[1..4]);    // ✅ Vector slice
    print_slice(&[10, 20]);   // ✅ Inline slice
}
```

---

## Slices Prevent Invalid References

Here's a powerful example of why slices are useful:

```rust
fn first_word_old(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    
    s.len()
}

fn main() {
    let mut s = String::from("hello world");
    let word_end = first_word_old(&s);  // Returns index 5
    
    s.clear();  // ❌ PROBLEM: now the string is empty!
                // But word_end = 5 no longer valid
    
    // word_end is now invalid, but Rust can't stop this
}
```

**With slices:**

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];  // Return slice
        }
    }
    
    &s[..]
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);  // Borrows s
    
    s.clear();  // ❌ COMPILE ERROR: can't clear while word borrows it
    
    println!("{}", word);
}
```

**Compiler error:**
```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
   --> src/main.rs:13:5
    |
12 |     let word = first_word(&s);
    |                           -- immutable borrow occurs here
13 |     s.clear();
    |     ^^^^^^^^^ mutable borrow occurs here
14 |     
15 |     println!("{}", word);
    |              ---- immutable borrow later used here
```

**The borrow checker prevents the bug!**

---

## Slice Patterns

You can destructure slices using patterns:

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    
    // Match patterns
    match &arr[..] {
        [] => println!("Empty"),
        [first] => println!("Single: {}", first),
        [first, rest @ ..] => println!("First: {}, Rest: {:?}", first, rest),
    }
}
```

**Output:**
```
First: 1, Rest: [2, 3, 4, 5]
```

---

## Mutable Slices

You can create mutable slices with `&mut [T]`:

```rust
fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    
    // Mutable slice
    let slice = &mut arr[1..4];
    slice[0] = 20;  // Modify element
    
    println!("Array: {:?}", arr);  // [1, 20, 3, 4, 5]
}
```

### Processing Mutable Slices

```rust
fn add_one(slice: &mut [i32]) {
    for element in slice {
        *element += 1;
    }
}

fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    add_one(&mut arr[1..4]);
    println!("Array: {:?}", arr);  // [1, 3, 4, 5, 5]
}
```

---

## Practical Examples

### Finding a Substring

```rust
fn find_substring(haystack: &str, needle: &str) -> Option<usize> {
    for i in 0..=haystack.len().saturating_sub(needle.len()) {
        if &haystack[i..i + needle.len()] == needle {
            return Some(i);
        }
    }
    None
}

fn main() {
    let text = "Hello, world!";
    match find_substring(text, "world") {
        Some(index) => println!("Found at index: {}", index),
        None => println!("Not found"),
    }
}
```

**Output:**
```
Found at index: 7
```

### Processing Lines

```rust
fn process_line(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}

fn main() {
    let text = "hello world from rust";
    let words = process_line(text);
    println!("Words: {:?}", words);
}
```

**Output:**
```
Words: ["hello", "world", "from", "rust"]
```

---

## Key Takeaways

✅ **Slices are references** to contiguous collections.

✅ **String slices (`&str`)** reference UTF-8 text.

✅ **Array slices (`&[T]`)** reference array/vector elements.

✅ **Slices don't own data** — they borrow it.

✅ **Slice syntax**: `&collection[start..end]` (end exclusive).

✅ **Omit boundaries** — `&arr[..5]`, `&arr[5..]`, `&arr[..]`.

✅ **Borrow checker protects** — slices prevent invalid references.

✅ **Mutable slices** with `&mut [T]`.

---

**Next:** [Lifetimes](lifetimes.md) — Ensuring references stay valid.
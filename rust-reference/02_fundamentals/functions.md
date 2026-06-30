# Functions

## Declaring Functions

### Basic Syntax

```rust
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // Function body
}
```

Every part is **required** — no defaults.

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y  // Return value (no semicolon!)
}

fn main() {
    let result = add(5, 6);
    println!("5 + 6 = {}", result);
}
```

**Output:**
```
5 + 6 = 11
```

### Return Values

**Explicit return** with `return` keyword:

```rust
fn divide(x: i32, y: i32) -> i32 {
    if y == 0 {
        return 0;  // Early return
    }
    x / y
}

fn main() {
    println!("10 / 2 = {}", divide(10, 2));  // 5
    println!("10 / 0 = {}", divide(10, 0));  // 0
}
```

**Implicit return** (preferred):

The last expression in a function (without semicolon) is the return value:

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y  // ✅ Returns x + y
}

fn add_bad(x: i32, y: i32) -> i32 {
    x + y;  // ❌ Returns () (unit), not x + y
}
```

### Functions with No Return Value

Omit `->` to return nothing (unit type `()`):

```rust
fn print_message(msg: &str) {
    println!("Message: {}", msg);
}

fn main() {
    print_message("Hello");  // No return value captured
}
```

Equivalent to:

```rust
fn print_message(msg: &str) -> () {
    println!("Message: {}", msg);
}
```

---

## Parameters and Arguments

### Multiple Parameters

```rust
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn main() {
    print_labeled_measurement(5, 'h');
    print_labeled_measurement(6, 'm');
}
```

**Output:**
```
The measurement is: 5h
The measurement is: 6m
```

### Type Annotations Required

Unlike some languages, Rust **requires type annotations** for every parameter:

```rust
fn add(x: i32, y: i32) -> i32 {  // Types are required
    x + y
}

// ❌ This won't work:
// fn add(x, y) {
//     x + y
// }
```

---

## Statements vs. Expressions

**Statements** perform actions but return nothing. **Expressions** return values.

### Statements

End with semicolon:

```rust
fn main() {
    let x = 5;
    let y = (let z = 6);  // ❌ ERROR: let is a statement
}
```

### Expressions

No semicolon at the end:

```rust
fn main() {
    let x = 5 + 6;              // ✅ Expression: 5 + 6
    let y = {
        let x = 3;
        x + 1                   // ✅ Expression
    };
    println!("y = {}", y);      // y = 4
    
    // With semicolon, it becomes a statement
    let z = {
        let x = 3;
        x + 1;                  // Now a statement (returns ())
    };
    println!("z = {:?}", z);    // z = ()
}
```

**Output:**
```
y = 4
z = ()
```

### if/else as Expression

```rust
fn main() {
    let number = 6;
    
    // if as expression
    let result = if number % 2 == 0 {
        "even"
    } else {
        "odd"
    };
    
    println!("Number is: {}", result);
}
```

**Output:**
```
Number is: even
```

---

## Practical Examples

### Calculator Functions

```rust
fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        return 0.0;
    }
    a / b
}

fn main() {
    println!("5 + 3 = {}", add(5.0, 3.0));
    println!("5 - 3 = {}", subtract(5.0, 3.0));
    println!("5 * 3 = {}", multiply(5.0, 3.0));
    println!("5 / 3 = {}", divide(5.0, 3.0));
}
```

### Factorial (Recursion)

```rust
fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

fn main() {
    println!("5! = {}", factorial(5));  // 120
}
```

### FizzBuzz

```rust
fn fizzbuzz(n: i32) {
    for i in 1..=n {
        let output = match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _ => i.to_string(),
        };
        println!("{}", output);
    }
}

fn main() {
    fizzbuzz(15);
}
```

---

## Diverging Functions

Functions that **never return** (infinite loop, panic, exit) have return type `!`:

```rust
fn diverge() -> ! {
    panic!("This function never returns");
}

fn infinite_loop() -> ! {
    loop {
        println!("Forever...");
    }
}

fn main() {
    // diverge();  // Would panic
    // infinite_loop();  // Would never stop
}
```

---

## Function Scope

Functions can be defined at module level or nested inside other functions:

```rust
fn outer() {
    println!("Outer function");
    
    fn inner() {
        println!("Inner function");
    }
    
    inner();  // ✅ Can call inner here
}

fn main() {
    outer();
    // inner();  // ❌ Can't call inner here (not in scope)
}
```

---

## Common Mistakes

### Forgetting Semicolon

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y   // ✅ No semicolon: returns x + y
}

fn add_wrong(x: i32, y: i32) -> i32 {
    x + y;  // ❌ With semicolon: returns (), not i32
}
```

**Compiler error:**
```
error[E0308]: mismatched types
   |
 2 | fn add_wrong(x: i32, y: i32) -> i32 {
   |    --------- expected `i32` because of return type
 3 |     x + y;
   |     ^^^^^^ expected `i32`, found `()`
```

### Missing Type Annotation

```rust
fn divide(x, y) {  // ❌ Missing types
    x / y
}

fn divide(x: i32, y: i32) -> i32 {  // ✅ Correct
    x / y
}
```

### Wrong Return Type

```rust
fn get_number() -> i32 {
    "42"  // ❌ Returns &str, not i32
}

fn get_number() -> i32 {
    42    // ✅ Correct
}
```

---

## Key Takeaways

✅ **Parameter types are required** — every parameter must have an explicit type.

✅ **Return type specified with `->` arrow** — omit for unit type ().

✅ **Last expression is return value** — use implicit return (no semicolon).

✅ **Statements end with semicolon**, **expressions don't**.

✅ **Use `return` for early exit**, but implicit return is idiomatic.

✅ **Functions can be nested** but are scoped to their containing function.

✅ **Diverging functions** return `!` (never return).

✅ **Compiler catches type mismatches** — error messages are helpful.

---

**Next:** [Control Flow](control_flow.md) — if/else, match, loops.
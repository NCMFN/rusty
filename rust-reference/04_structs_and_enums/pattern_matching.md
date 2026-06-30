# Pattern Matching

## The match Expression

**match** is a powerful control flow construct for handling all variants of an enum.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn coin_value(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("Penny: {}", coin_value(Coin::Penny));
}
```

---

## Patterns in match

### Extracting Values

```rust
enum Result {
    Ok(String),
    Err(String),
}

fn main() {
    let result = Result::Ok(String::from("Success!"));
    
    match result {
        Result::Ok(msg) => println!("Success: {}", msg),
        Result::Err(err) => println!("Error: {}", err),
    }
}
```

**Output:**
```
Success: Success!
```

### Destructuring Tuples

```rust
fn main() {
    let point = (3, 4);
    
    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("On x-axis: {}", x),
        (0, y) => println!("On y-axis: {}", y),
        (x, y) => println!("At ({}, {})", x, y),
    }
}
```

**Output:**
```
At (3, 4)
```

### Destructuring Structs

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3, y: 4 };
    
    match p {
        Point { x: 0, y: 0 } => println!("Origin"),
        Point { x, y: 0 } => println!("On x-axis at {}", x),
        Point { x: 0, y } => println!("On y-axis at {}", y),
        Point { x, y } => println!("At ({}, {})", x, y),
    }
}
```

### Guard Clauses

Add conditions with `if`:

```rust
fn main() {
    let x = 5;
    
    match x {
        1 | 2 | 3 => println!("One, two, or three"),
        4..=6 if x % 2 == 0 => println!("Even in range 4-6"),
        4..=6 => println!("Odd in range 4-6"),
        _ => println!("Other"),
    }
}
```

**Output:**
```
Even in range 4-6
```

---

## if let Expression

**if let** is a concise way to match a single pattern.

### Basic if let

```rust
fn main() {
    let x = Some(5);
    
    // Long form
    match x {
        Some(val) => println!("Value: {}", val),
        None => {},
    }
    
    // Concise form
    if let Some(val) = x {
        println!("Value: {}", val);
    }
}
```

### if let with else

```rust
fn main() {
    let x = Some(5);
    
    if let Some(val) = x {
        println!("Value: {}", val);
    } else {
        println!("None");
    }
}
```

### if let with else if

```rust
enum Color {
    Red,
    Green(u8),
    Blue(u8, u8),
}

fn main() {
    let color = Color::Green(100);
    
    if let Color::Red = color {
        println!("Red");
    } else if let Color::Green(g) = color {
        println!("Green: {}", g);
    } else if let Color::Blue(b, g) = color {
        println!("Blue: ({}, {})", b, g);
    }
}
```

**Output:**
```
Green: 100
```

---

## while let Loop

**while let** repeatedly matches a pattern.

```rust
fn main() {
    let mut stack = vec![1, 2, 3, 4, 5];
    
    while let Some(val) = stack.pop() {
        println!("Popped: {}", val);
    }
}
```

**Output:**
```
Popped: 5
Popped: 4
Popped: 3
Popped: 2
Popped: 1
```

---

## Exhaustiveness

**match** must be exhaustive — all patterns handled:

```rust
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let x = Some(5);
    
    // ❌ ERROR: missing None case
    match x {
        Some(val) => println!("Value: {}", val),
    }
}
```

**Compiler error:**
```
error[E0004]: non-exhaustive patterns: `None` not covered
  --> src/main.rs:6:11
   |
6  |     match x {
   |           ^
   |
   = note: `Option::<i32>::Some(_)` not covered
   = note: the matched value is of type `Option<i32>`
```

**Fix**: Add missing pattern:

```rust
match x {
    Some(val) => println!("Value: {}", val),
    None => println!("No value"),
}
```

### Catch-all with _

```rust
match x {
    1 => println!("One"),
    2 => println!("Two"),
    _ => println!("Other"),  // Catch-all
}
```

---

## Advanced Patterns

### Ranges

```rust
fn main() {
    let x = 5;
    
    match x {
        1..=5 => println!("Between 1 and 5"),
        6..=10 => println!("Between 6 and 10"),
        _ => println!("Other"),
    }
}
```

### Multiple Patterns

```rust
fn main() {
    let x = 2;
    
    match x {
        1 | 2 | 3 => println!("One, two, or three"),
        _ => println!("Other"),
    }
}
```

### Nested Patterns

```rust
fn main() {
    let point = Some((1, 2));
    
    match point {
        Some((x, y)) if x == y => println!("x and y are equal"),
        Some((x, y)) => println!("At ({}, {})", x, y),
        None => println!("None"),
    }
}
```

### Binding with @

```rust
fn main() {
    let x = 5;
    
    match x {
        n @ 1..=5 => println!("Got {}, in range 1-5", n),
        _ => println!("Other"),
    }
}
```

---

## Practical Examples

### Parsing Commands

```rust
enum Command {
    Quit,
    Move { direction: String, distance: u32 },
    Help,
}

fn execute(cmd: Command) {
    match cmd {
        Command::Quit => println!("Quitting..."),
        Command::Move { direction, distance } => {
            println!("Moving {} in direction {}", distance, direction);
        }
        Command::Help => println!("Available: quit, move, help"),
    }
}

fn main() {
    let cmd = Command::Move {
        direction: String::from("north"),
        distance: 10,
    };
    execute(cmd);
}
```

### Error Handling

```rust
fn parse_number(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(num) => {
            if num > 0 {
                Ok(num)
            } else {
                Err(String::from("Not positive"))
            }
        }
        Err(_) => Err(String::from("Not a number")),
    }
}

fn main() {
    match parse_number("42") {
        Ok(num) => println!("Parsed: {}", num),
        Err(err) => println!("Error: {}", err),
    }
}
```

### Processing Collections

```rust
fn main() {
    let values = vec![Some(1), None, Some(3), None, Some(5)];
    
    for val in values {
        if let Some(n) = val {
            println!("Value: {}", n);
        }
    }
}
```

---

## Key Takeaways

✅ **match is exhaustive** — all patterns must be handled.

✅ **Destructuring extracts values** from enums, tuples, structs.

✅ **Guard clauses** add conditions with `if`.

✅ **if let** for concise single-pattern matching.

✅ **while let** for repeated pattern matching.

✅ **_ catch-all** pattern for remaining cases.

✅ **Pattern combinations** — ranges, multiple values, nesting.

✅ **Compiler enforces** exhaustiveness and correctness.

---

**Next:** [Result and Error Handling](result_and_error.md) — Proper error handling.
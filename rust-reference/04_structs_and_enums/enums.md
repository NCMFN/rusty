# Enums

## What Are Enums?

An **enum** (enumeration) defines a type with multiple possible variants.

```rust
enum Direction {
    North,
    South,
    East,
    West,
}
```

---

## Basic Enums

### Declaring and Using Enums

```rust
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let color = Color::Red;
    // Use with patterns (next section)
}
```

### Enums with Data

Variants can carry associated data:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("hello"));
    let msg4 = Message::ChangeColor(255, 128, 0);
}
```

Variants can have:
- **No data**: `Quit`
- **Struct data**: `Move { x, y }`
- **Tuple data**: `Write(String)`, `ChangeColor(i32, i32, i32)`

---

## Option<T>

Rust's **Option** enum represents a value that might be present or absent.

```rust
pub enum Option<T> {
    Some(T),
    None,
}
```

### Using Option

```rust
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num);
        }
    }
    None
}

fn main() {
    let numbers = vec![1, 3, 5, 6, 7];
    
    match find_first_even(&numbers) {
        Some(num) => println!("Found: {}", num),
        None => println!("No even number found"),
    }
}
```

**Output:**
```
Found: 6
```

### Option Methods

```rust
fn main() {
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;
    
    // is_some(), is_none()
    println!("x.is_some(): {}", x.is_some());  // true
    println!("y.is_none(): {}", y.is_none());  // true
    
    // unwrap() - panics if None
    println!("x.unwrap(): {}", x.unwrap());    // 5
    // println!("y.unwrap(): {}", y.unwrap());  // PANIC
    
    // unwrap_or() - default value
    println!("y.unwrap_or(0): {}", y.unwrap_or(0));  // 0
    
    // map() - transform value
    let x_squared = x.map(|val| val * val);
    println!("x_squared: {:?}", x_squared);  // Some(25)
}
```

---

## Result<T, E>

Rust's **Result** enum represents success or failure.

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Using Result

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
```

**Output:**
```
Result: 5
Error: Division by zero
```

### Result Methods

```rust
fn main() {
    let x: Result<i32, String> = Ok(5);
    let y: Result<i32, String> = Err(String::from("error"));
    
    // is_ok(), is_err()
    println!("x.is_ok(): {}", x.is_ok());    // true
    println!("y.is_err(): {}", y.is_err());  // true
    
    // unwrap() - panics if Err
    println!("x.unwrap(): {}", x.unwrap());  // 5
    
    // unwrap_or() - default value
    println!("y.unwrap_or(0): {}", y.unwrap_or(0));  // 0
    
    // map() - transform Ok value
    let x_squared = x.map(|val| val * val);
    println!("x_squared: {:?}", x_squared);  // Ok(25)
}
```

---

## Enum Methods

### Implementing Methods on Enums

```rust
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn is_warm(&self) -> bool {
        matches!(self, Color::Red)
    }
    
    fn name(&self) -> &str {
        match self {
            Color::Red => "red",
            Color::Green => "green",
            Color::Blue => "blue",
        }
    }
}

fn main() {
    let color = Color::Red;
    println!("Name: {}", color.name());
    println!("Warm: {}", color.is_warm());
}
```

---

## Generic Enums

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Option<T> {
    Some(T),
    None,
}

// Custom generic enum
enum Either<L, R> {
    Left(L),
    Right(R),
}

fn main() {
    let e1: Either<i32, String> = Either::Left(5);
    let e2: Either<i32, String> = Either::Right(String::from("error"));
}
```

---

## Practical Examples

### Traffic Light

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn time_to_wait(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 25,
        }
    }
}

fn main() {
    let light = TrafficLight::Red;
    println!("Wait {} seconds", light.time_to_wait());
}
```

### HTTP Response

```rust
enum Response {
    Ok(String),
    Created(String),
    BadRequest(String),
    NotFound,
    ServerError(String),
}

impl Response {
    fn status_code(&self) -> u32 {
        match self {
            Response::Ok(_) => 200,
            Response::Created(_) => 201,
            Response::BadRequest(_) => 400,
            Response::NotFound => 404,
            Response::ServerError(_) => 500,
        }
    }
}

fn main() {
    let response = Response::Ok(String::from("Success"));
    println!("Status: {}", response.status_code());
}
```

### State Machine

```rust
enum PlayerState {
    Idle,
    Running(u32),  // speed
    Jumping { height: u32, falling: bool },
}

impl PlayerState {
    fn describe(&self) -> String {
        match self {
            PlayerState::Idle => "Standing still".to_string(),
            PlayerState::Running(speed) => format!("Running at {} mph", speed),
            PlayerState::Jumping { height, falling } => {
                if *falling {
                    format!("Falling from {} units", height)
                } else {
                    format!("Jumping to {} units", height)
                }
            }
        }
    }
}

fn main() {
    let state = PlayerState::Running(10);
    println!("{}", state.describe());
}
```

---

## Key Takeaways

✅ **Enums define types with multiple variants**.

✅ **Variants can carry data** (struct, tuple, or no data).

✅ **Option<T>** for optional values (Some/None).

✅ **Result<T, E>** for success/failure (Ok/Err).

✅ **impl blocks** add methods to enums.

✅ **Generic enums** work with any types.

✅ **No null pointers** — Option forces explicit handling.

✅ **Type safety** — compiler ensures all variants handled.

---

**Next:** [Pattern Matching](pattern_matching.md) — Destructuring and matching enums.
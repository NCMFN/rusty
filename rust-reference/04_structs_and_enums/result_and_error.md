# Result and Error Handling

## The Result Type

**Result** is Rust's primary error handling mechanism.

```rust
pub enum Result<T, E> {
    Ok(T),      // Success with value
    Err(E),     // Failure with error
}
```

---

## Handling Results

### Match Expression

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    
    match f {
        Ok(file) => println!("File opened successfully"),
        Err(err) => println!("Error: {}", err),
    }
}
```

### unwrap()

Panic if Result is Err:

```rust
fn main() {
    let f = File::open("hello.txt").unwrap();  // Panics if file not found
}
```

### expect()

Panic with custom message:

```rust
fn main() {
    let f = File::open("hello.txt")
        .expect("Failed to open hello.txt");
}
```

### unwrap_or()

Provide default value:

```rust
fn main() {
    let x: Result<i32, String> = Ok(5);
    println!("{}", x.unwrap_or(0));  // 5
    
    let y: Result<i32, String> = Err(String::from("error"));
    println!("{}", y.unwrap_or(0));  // 0
}
```

---

## The ? Operator

The **?** operator propagates errors up the call stack.

### Without ?

```rust
use std::fs::File;
use std::io::Read;

fn read_file() -> Result<String, std::io::Error> {
    match File::open("hello.txt") {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => Ok(contents),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}
```

### With ?

```rust
use std::fs::File;
use std::io::Read;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("hello.txt")?;  // Early return on Err
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;      // Early return on Err
    Ok(contents)                              // Return Ok
}
```

**Much cleaner!**

### How ? Works

The `?` operator:
1. If Result is `Ok(T)`, unwraps to `T` and continues
2. If Result is `Err(E)`, returns `Err(E)` from function

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Division by zero"));
    }
    Ok(a / b)
}

fn calculate() -> Result<i32, String> {
    let x = divide(10, 2)?;   // Ok(5) -> x = 5; continues
    let y = divide(20, 0)?;   // Err(...) -> returns Err immediately
    Ok(x + y)                 // Not reached
}

fn main() {
    match calculate() {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}
```

**Output:**
```
Error: Division by zero
```

---

## Custom Error Types

### Implementing std::error::Error

```rust
use std::fmt;

#[derive(Debug)]
enum ParseError {
    InvalidFormat,
    OutOfRange,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidFormat => write!(f, "Invalid format"),
            ParseError::OutOfRange => write!(f, "Out of range"),
        }
    }
}

impl std::error::Error for ParseError {}

fn parse_age(s: &str) -> Result<u32, ParseError> {
    let age: u32 = s.parse()
        .map_err(|_| ParseError::InvalidFormat)?;
    
    if age > 150 {
        return Err(ParseError::OutOfRange);
    }
    
    Ok(age)
}

fn main() {
    match parse_age("25") {
        Ok(age) => println!("Age: {}", age),
        Err(e) => println!("Error: {}", e),
    }
}
```

### Using thiserror Crate

Add to `Cargo.toml`:
```toml
[dependencies]
thiserror = "1.0"
```

Then:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid format")]
    InvalidFormat,
    
    #[error("Out of range")]
    OutOfRange,
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

fn parse_age(s: &str) -> Result<u32, ParseError> {
    let age: u32 = s.parse()
        .map_err(|_| ParseError::InvalidFormat)?;
    
    if age > 150 {
        return Err(ParseError::OutOfRange);
    }
    
    Ok(age)
}
```

---

## Error Propagation Patterns

### map_err()

Transform error type:

```rust
fn parse_number(s: &str) -> Result<i32, String> {
    s.parse()
        .map_err(|_| String::from("Failed to parse"))
}

fn main() {
    match parse_number("abc") {
        Ok(num) => println!("Number: {}", num),
        Err(e) => println!("Error: {}", e),
    }
}
```

### ok_or()

Convert Option to Result:

```rust
fn main() {
    let x = Some(5);
    let result: Result<i32, String> = x.ok_or(String::from("Not found"));
    println!("{:?}", result);  // Ok(5)
    
    let y: Option<i32> = None;
    let result: Result<i32, String> = y.ok_or(String::from("Not found"));
    println!("{:?}", result);  // Err("Not found")
}
```

### and_then()

Chain operations:

```rust
fn main() {
    let x: Result<i32, String> = Ok(5);
    
    let result = x
        .and_then(|val| {
            if val > 0 {
                Ok(val * 2)
            } else {
                Err(String::from("Not positive"))
            }
        });
    
    println!("{:?}", result);  // Ok(10)
}
```

---

## Practical Examples

### File Reading with Error Handling

```rust
use std::fs;
use std::io;

fn read_config() -> Result<String, io::Error> {
    let config = fs::read_to_string("config.txt")?;
    Ok(config)
}

fn main() {
    match read_config() {
        Ok(config) => println!("Config: {}", config),
        Err(e) => eprintln!("Error reading config: {}", e),
    }
}
```

### Parsing CSV

```rust
use std::num::ParseIntError;

#[derive(Debug)]
struct Record {
    id: u32,
    name: String,
    age: u32,
}

fn parse_record(line: &str) -> Result<Record, ParseIntError> {
    let mut parts = line.split(',');
    
    let id = parts.next()
        .ok_or_else(|| ParseIntError::new())?
        .trim()
        .parse()?;
    
    let name = parts.next()
        .ok_or_else(|| ParseIntError::new())?
        .trim()
        .to_string();
    
    let age = parts.next()
        .ok_or_else(|| ParseIntError::new())?
        .trim()
        .parse()?;
    
    Ok(Record { id, name, age })
}

fn main() {
    match parse_record("1,Alice,30") {
        Ok(record) => println!("Record: {:?}", record),
        Err(e) => println!("Parse error: {}", e),
    }
}
```

### Fallible Vector Operations

```rust
fn sum_or_error(numbers: &[i32]) -> Result<i32, String> {
    if numbers.is_empty() {
        return Err(String::from("Empty vector"));
    }
    
    let sum: i32 = numbers.iter().sum();
    Ok(sum)
}

fn main() {
    match sum_or_error(&[1, 2, 3]) {
        Ok(sum) => println!("Sum: {}", sum),
        Err(e) => println!("Error: {}", e),
    }
    
    match sum_or_error(&[]) {
        Ok(sum) => println!("Sum: {}", sum),
        Err(e) => println!("Error: {}", e),
    }
}
```

---

## When to Use Panic vs Error

### Use panic!() when:
- Programming error (logic bug)
- Unrecoverable situation
- Assertions

```rust
assert!(x > 0, "x must be positive");
panic!("This should never happen");
```

### Use Result when:
- Operation might fail for valid reasons
- Caller should handle the error
- Different recovery strategies

```rust
fn open_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)  // Caller handles failure
}
```

---

## Key Takeaways

✅ **Result<T, E>** represents success or failure.

✅ **? operator** propagates errors cleanly.

✅ **unwrap()** for simple cases (panics on Err).

✅ **expect()** with custom message.

✅ **Custom error types** implement std::error::Error.

✅ **map_err()** transforms error types.

✅ **Chaining methods** — and_then(), ok_or(), etc.

✅ **Compiler enforces** error handling.

---

**Next:** [Collections](../05_collections/vec.md) — Vec, HashMap, String.
# Control Flow

## if/else Expressions

### Basic if/else

```rust
fn main() {
    let number = 6;
    
    if number % 2 == 0 {
        println!("Number is even");
    } else {
        println!("Number is odd");
    }
}
```

**Output:**
```
Number is even
```

### Condition Requirements

The condition **must be a bool**, not a number (unlike C):

```rust
fn main() {
    let number = 3;
    
    if number {  // ❌ ERROR: expected bool, found integer
        println!("Number is not zero");
    }
}
```

**Compiler error:**
```
error[E0308]: mismatched types
   |
 3 |     if number {
   |        ^^^^^^ expected `bool`, found `i32`
```

### Multiple Conditions with else if

```rust
fn main() {
    let number = 6;
    
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is odd");
    }
}
```

**Output:**
```
Number is divisible by 2
```

### if as Expression

`if` blocks return a value:

```rust
fn main() {
    let number = 6;
    
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

**Type consistency required:**

```rust
fn main() {
    let number = 6;
    
    // ❌ ERROR: incompatible types
    let result = if number % 2 == 0 {
        5
    } else {
        "six"  // Number vs. string
    };
}
```

---

## match Expression

`match` is Rust's powerful pattern matching construct. It's like a switch statement, but more powerful.

### Basic match

```rust
fn main() {
    let number = 3;
    
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        _ => println!("Something else"),  // Default case
    }
}
```

**Output:**
```
Three
```

### match with Multiple Values

```rust
fn main() {
    let number = 2;
    
    match number {
        1 | 2 | 3 => println!("One, two, or three"),
        4 | 5 | 6 => println!("Four, five, or six"),
        _ => println!("Something else"),
    }
}
```

**Output:**
```
One, two, or three
```

### match with Ranges

```rust
fn main() {
    let number = 7;
    
    match number {
        1..=5 => println!("One through five"),
        6..=10 => println!("Six through ten"),
        _ => println!("Something else"),
    }
}
```

**Output:**
```
Six through ten
```

### match Returns Values

```rust
fn main() {
    let number = 6;
    
    let description = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        _ => "other",
    };
    
    println!("Number {}: {}", number, description);
}
```

**Output:**
```
Number 6: six
```

### Complex Patterns in match

```rust
fn main() {
    let point = (3, 4);
    
    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("On x-axis at {}", x),
        (0, y) => println!("On y-axis at {}", y),
        (x, y) => println!("At ({}, {})", x, y),
    }
}
```

**Output:**
```
At (3, 4)
```

---

## Loops

### Infinite Loop

```rust
fn main() {
    let mut count = 0;
    
    loop {
        count += 1;
        println!("Count: {}", count);
        
        if count == 3 {
            break;  // Exit loop
        }
    }
}
```

**Output:**
```
Count: 1
Count: 2
Count: 3
```

### Loop Returns Value

```rust
fn main() {
    let mut count = 0;
    
    let result = loop {
        count += 1;
        
        if count == 5 {
            break count * 2;  // Return value
        }
    };
    
    println!("Result: {}", result);  // Result: 10
}
```

### Nested Loops with Labels

```rust
fn main() {
    'outer: loop {
        println!("Outer loop");
        
        for i in 0..3 {
            println!("Inner loop: {}", i);
            
            if i == 1 {
                break 'outer;  // Break outer loop
            }
        }
    }
}
```

**Output:**
```
Outer loop
Inner loop: 0
Inner loop: 1
```

### while Loop

```rust
fn main() {
    let mut number = 3;
    
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    println!("LIFTOFF!");
}
```

**Output:**
```
3!
2!
1!
LIFTOFF!
```

### for Loop (Ranges)

```rust
fn main() {
    // Exclusive range (0, 1, 2)
    for i in 0..3 {
        println!("i = {}", i);
    }
    
    println!("---");
    
    // Inclusive range (1, 2, 3)
    for i in 1..=3 {
        println!("i = {}", i);
    }
}
```

**Output:**
```
i = 0
i = 1
i = 2
---
i = 1
i = 2
i = 3
```

### for Loop (Collections)

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    
    for element in &a {
        println!("Element: {}", element);
    }
}
```

**Output:**
```
Element: 10
Element: 20
Element: 30
Element: 40
Element: 50
```

### break and continue

```rust
fn main() {
    for i in 0..10 {
        if i == 2 {
            continue;  // Skip this iteration
        }
        
        if i == 5 {
            break;  // Exit loop
        }
        
        println!("i = {}", i);
    }
}
```

**Output:**
```
i = 0
i = 1
i = 3
i = 4
```

---

## Practical Examples

### Countdown Timer

```rust
fn main() {
    let mut count = 10;
    
    loop {
        match count {
            0 => {
                println!("Liftoff!");
                break;
            }
            1 => println!("1 second remaining!"),
            _ => println!("{}...", count),
        }
        count -= 1;
    }
}
```

### Number Guessing Game (Simplified)

```rust
fn main() {
    let secret = 42;
    
    loop {
        println!("Guess the number:");
        
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
        
        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        }
    }
}
```

---

## Key Takeaways

✅ **Conditions must be bool** — Rust won't coerce integers.

✅ **if/else are expressions** — they return values.

✅ **match is powerful** — pattern matching, multiple values, ranges.

✅ **loop is infinite** — use `break` to exit.

✅ **while loops** for condition-based repetition.

✅ **for loops** preferred for iterating collections and ranges.

✅ **break returns values** — useful for getting results from loops.

✅ **Loop labels** for breaking nested loops.

✅ **continue skips** to the next iteration.

---

**Next:** [Comments and Documentation](comments_and_docs.md) — Write clear, documented code.
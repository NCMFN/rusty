# Structs

## What Are Structs?

A **struct** (structure) is a custom data type that groups related data together.

```rust
struct Person {
    name: String,
    age: u32,
    email: String,
}
```

---

## Struct Types

### 1. Struct with Named Fields

```rust
struct Person {
    name: String,
    age: u32,
    email: String,
}

fn main() {
    // Create instance
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
    };
    
    // Access fields
    println!("Name: {}", alice.name);
    println!("Age: {}", alice.age);
}
```

### 2. Tuple Struct

Structs without field names:

```rust
struct Color(i32, i32, i32);  // RGB
struct Point(f64, f64);        // 2D point

fn main() {
    let red = Color(255, 0, 0);
    let origin = Point(0.0, 0.0);
    
    println!("Red: ({}, {}, {})", red.0, red.1, red.2);
    println!("Origin: ({}, {})", origin.0, origin.1);
}
```

### 3. Unit Struct

Structs with no fields:

```rust
struct Marker;

fn main() {
    let m = Marker;
}
```

Useful for marking types or implementing traits.

---

## Creating Instances

### Field Shorthand

```rust
struct Person {
    name: String,
    age: u32,
}

fn create_person(name: String, age: u32) -> Person {
    // Longhand
    Person { name: name, age: age }
    
    // Shorthand (same meaning)
    Person { name, age }
}

fn main() {
    let alice = create_person(String::from("Alice"), 30);
}
```

### Struct Update Syntax

```rust
struct Person {
    name: String,
    age: u32,
    email: String,
}

fn main() {
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
    };
    
    // Create similar instance with some fields changed
    let bob = Person {
        name: String::from("Bob"),
        ..alice  // Copy remaining fields from alice
    };
}
```

---

## Methods and Associated Functions

### Using impl Blocks

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (no &self)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    // Method (takes &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method (takes &mut self)
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
    
    // Method (takes self, consumes it)
    fn into_square(self) -> Rectangle {
        let size = self.width.min(self.height);
        Rectangle { width: size, height: size }
    }
}

fn main() {
    // Associated function call
    let mut rect = Rectangle::new(30, 50);
    
    // Method calls
    println!("Area: {}", rect.area());
    
    rect.scale(2);
    println!("After scale: {} x {}", rect.width, rect.height);
    
    let square = rect.into_square();
    println!("Square: {} x {}", square.width, square.height);
}
```

### Multiple impl Blocks

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
}
```

---

## Self Reference Types

### &self (Immutable Borrow)

Don't take ownership, allow multiple calls:

```rust
struct Counter {
    count: i32,
}

impl Counter {
    fn get(&self) -> i32 {
        self.count
    }
}

fn main() {
    let counter = Counter { count: 5 };
    println!("{}", counter.get());
    println!("{}", counter.get());  // Still valid
}
```

### &mut self (Mutable Borrow)

Allow mutations:

```rust
struct Counter {
    count: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.count += 1;
    }
}

fn main() {
    let mut counter = Counter { count: 0 };
    counter.increment();
    println!("Count: {}", counter.count);
}
```

### self (Ownership)

Consume the struct:

```rust
struct Player {
    name: String,
}

impl Player {
    fn take_damage(self) -> String {
        format!("{} is defeated!", self.name)
    }
}

fn main() {
    let player = Player { name: String::from("Hero") };
    let result = player.take_damage();
    println!("{}", result);
    // println!("Player: {:?}", player);  // Error: player consumed
}
```

---

## Generic Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 1.0, y: 4.0 };
    
    println!("p1.x = {}", p1.x());
    println!("p2.x = {}", p2.x());
}
```

### Generic with Different Types

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Pair<T, U> {
        Pair { first, second }
    }
}

fn main() {
    let pair = Pair::new(5, "hello");
}
```

---

## Practical Examples

### User struct with Methods

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

impl User {
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            active: true,
            sign_in_count: 1,
        }
    }
    
    fn sign_in(&mut self) {
        self.sign_in_count += 1;
    }
    
    fn display_info(&self) {
        println!("User: {}", self.username);
        println!("Email: {}", self.email);
        println!("Active: {}", self.active);
        println!("Sign-ins: {}", self.sign_in_count);
    }
}

fn main() {
    let mut user = User::new(
        String::from("alice"),
        String::from("alice@example.com"),
    );
    
    user.sign_in();
    user.display_info();
}
```

### Rectangle with Trait Implementation

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 20, height: 40 };
    let rect3 = Rectangle { width: 40, height: 60 };
    
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {}", rect1.can_hold(&rect3));
}
```

---

## Debugging Structs

### Derive Debug Trait

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("rect: {:?}", rect);      // Single-line
    println!("rect: {:#?}", rect);     // Pretty-printed
}
```

**Output:**
```
rect: Rectangle { width: 30, height: 50 }
rect: Rectangle {
    width: 30,
    height: 50,
}
```

---

## Key Takeaways

✅ **Named structs** group related data with field names.

✅ **Tuple structs** are lightweight with positional fields.

✅ **Unit structs** have no fields (useful for markers).

✅ **impl blocks** define methods and associated functions.

✅ **&self, &mut self, self** determine ownership and mutability.

✅ **Generic structs** work with any type.

✅ **Field shorthand** and **update syntax** make code concise.

✅ **Derive Debug** for easy printing.

---

**Next:** [Enums](enums.md) — Creating types with multiple variants.
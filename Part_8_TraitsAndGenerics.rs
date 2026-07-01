// ============================================================================
// PART 8: TRAITS AND GENERICS IN RUST
// ============================================================================
// Traits: Define shared behavior that types can implement
// Generics: Write code that works with multiple types
// Together they enable polymorphism and code reuse in Rust
// ============================================================================

use std::fmt;
use std::cmp::Ordering;

// ============================================================================
// 1. TRAITS - DEFINING SHARED BEHAVIOR
// ============================================================================

trait Drawable {
    fn draw(&self);
}

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle {}x{}", self.width, self.height);
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn traits_basics() {
    println!("\n=== 1. TRAITS - BASIC IMPLEMENTATION ===\n");

    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 4.0, height: 3.0 };

    circle.draw();
    rectangle.draw();

    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rectangle.area());
}

// ============================================================================
// 2. TRAIT METHODS WITH DEFAULT IMPLEMENTATIONS
// ============================================================================

trait Animal {
    fn name(&self) -> &str;
    
    fn make_sound(&self) {
        println!("{} makes a sound", self.name());
    }
    
    fn sleep(&self) {
        println!("{} is sleeping", self.name());
    }
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn make_sound(&self) {
        println!("{} barks: Woof!", self.name());
    }
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn make_sound(&self) {
        println!("{} meows: Meow!", self.name());
    }
}

fn trait_default_implementations() {
    println!("\n=== 2. TRAIT DEFAULT IMPLEMENTATIONS ===\n");

    let dog = Dog { name: "Rex".to_string() };
    let cat = Cat { name: "Whiskers".to_string() };

    dog.make_sound();
    dog.sleep();

    cat.make_sound();
    cat.sleep();
}

// ============================================================================
// 3. TRAIT OBJECTS (DYNAMIC DISPATCH)
// ============================================================================

fn print_shape_info(shape: &dyn Drawable) {
    shape.draw();
}

fn trait_objects() {
    println!("\n=== 3. TRAIT OBJECTS ===\n");

    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle { width: 2.0, height: 4.0 };

    print_shape_info(&circle);
    print_shape_info(&rectangle);

    // Vector of trait objects
    let shapes: Vec<&dyn Drawable> = vec![&circle, &rectangle];
    
    println!("Drawing all shapes:");
    for shape in shapes {
        shape.draw();
    }
}

// ============================================================================
// 4. GENERICS - BASIC USAGE
// ============================================================================

fn print_item<T: fmt::Display>(item: T) {
    println!("Item: {}", item);
}

fn get_max<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair { x, y }
    }
}

impl<T: fmt::Display> Pair<T> {
    fn print(&self) {
        println!("Pair: ({}, {})", self.x, self.y);
    }
}

fn generics_basics() {
    println!("\n=== 4. GENERICS - BASIC USAGE ===\n");

    print_item(42);
    print_item("Hello");
    print_item(3.14);

    println!("Max of 5 and 10: {}", get_max(5, 10));
    println!("Max of 3.14 and 2.71: {}", get_max(3.14, 2.71));

    let pair = Pair::new(10, 20);
    pair.print();

    let str_pair = Pair::new("Hello", "World");
    str_pair.print();
}

// ============================================================================
// 5. TRAIT BOUNDS
// ============================================================================

trait Summarizable {
    fn summary(&self) -> String;
}

struct NewsArticle {
    title: String,
    author: String,
    content: String,
}

struct Tweet {
    author: String,
    content: String,
    retweets: u32,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.author, &self.content[..50])
    }
}

// Function with trait bound
fn notify<T: Summarizable>(item: &T) {
    println!("Breaking news! {}", item.summary());
}

// Multiple trait bounds
fn print_summary<T: Summarizable + fmt::Display>(item: &T) {
    println!("Summary: {}", item.summary());
}

// Using where clause for complex bounds
fn print_all_values<T>(items: &[T])
where
    T: fmt::Display + fmt::Debug,
{
    for item in items {
        println!("Display: {}, Debug: {:?}", item, item);
    }
}

fn trait_bounds() {
    println!("\n=== 5. TRAIT BOUNDS ===\n");

    let article = NewsArticle {
        title: "Breaking News".to_string(),
        author: "John Doe".to_string(),
        content: "Important content...".to_string(),
    };

    let tweet = Tweet {
        author: "Jane Smith".to_string(),
        content: "This is a tweet about Rust programming".to_string(),
        retweets: 42,
    };

    notify(&article);
    notify(&tweet);

    let numbers = vec![1, 2, 3, 4, 5];
    print_all_values(&numbers);
}

// ============================================================================
// 6. ASSOCIATED TYPES IN TRAITS
// ============================================================================

trait Container {
    type Item;

    fn get(&self) -> &Self::Item;
    fn put(&mut self, item: Self::Item);
}

struct Box<T> {
    item: Option<T>,
}

impl<T> Container for Box<T> {
    type Item = T;

    fn get(&self) -> &Self::Item {
        self.item.as_ref().unwrap()
    }

    fn put(&mut self, item: Self::Item) {
        self.item = Some(item);
    }
}

fn associated_types() {
    println!("\n=== 6. ASSOCIATED TYPES ===\n");

    let mut box_int = Box { item: None };
    box_int.put(42);
    println!("Box contains: {}", box_int.get());

    let mut box_str = Box { item: None };
    box_str.put("Hello");
    println!("Box contains: {}", box_str.get());
}

// ============================================================================
// 7. GENERIC STRUCTS WITH MULTIPLE TYPE PARAMETERS
// ============================================================================

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Point { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

impl<T: fmt::Display, U: fmt::Display> Point<T, U> {
    fn print(&self) {
        println!("Point: ({}, {})", self.x, self.y);
    }
}

// Concrete method for specific type
impl Point<f64, f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn generic_multiple_types() {
    println!("\n=== 7. MULTIPLE TYPE PARAMETERS ===\n");

    let p1 = Point::new(5, 10);
    println!("p1.x = {}", p1.x());

    let p2 = Point::new(1.5, 2.5);
    p2.print();

    println!("Distance from origin: {}", p2.distance_from_origin());

    let p3 = Point::new("Hello", 42);
    p3.print();
}

// ============================================================================
// 8. GENERIC TRAITS WITH STATIC DISPATCH
// ============================================================================

trait Iterator_Custom<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator_Custom<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn generic_traits() {
    println!("\n=== 8. GENERIC TRAITS ===\n");

    let mut counter = Counter::new(5);
    while let Some(value) = counter.next() {
        println!("Count: {}", value);
    }
}

// ============================================================================
// 9. TRAIT DERIVATION
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Person {
    name: String,
    age: u32,
}

// Manual implementation example (not using derive)
#[derive(Debug)]
struct Point2D {
    x: i32,
    y: i32,
}

impl PartialEq for Point2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn trait_derivation() {
    println!("\n=== 9. TRAIT DERIVATION ===\n");

    let p1 = Point2D { x: 1, y: 2 };
    let p2 = Point2D { x: 1, y: 2 };
    let p3 = Point2D { x: 3, y: 4 };

    println!("p1 == p2: {}", p1 == p2);
    println!("p1 == p3: {}", p1 == p3);
    println!("Debug format: {:?}", p1);
}

// ============================================================================
// 10. RETURN TYPE WITH IMPL TRAIT
// ============================================================================

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Return impl Trait (static dispatch)
fn get_shape_by_size(size: f64) -> impl Shape {
    if size > 5.0 {
        Rectangle { width: size, height: size }
    } else {
        Circle { radius: size }
    }
}

fn impl_trait() {
    println!("\n=== 10. IMPL TRAIT ===\n");

    let shape1 = get_shape_by_size(3.0);
    println!("Shape 1 area: {}", shape1.area());

    let shape2 = get_shape_by_size(10.0);
    println!("Shape 2 area: {}", shape2.area());
}

// ============================================================================
// 11. PRACTICAL EXAMPLE: GENERIC ALGORITHM
// ============================================================================

fn find<T: PartialEq>(items: &[T], target: &T) -> Option<usize> {
    for (i, item) in items.iter().enumerate() {
        if item == target {
            return Some(i);
        }
    }
    None
}

fn find_max<T: PartialOrd>(items: &[T]) -> Option<&T> {
    items.iter().max_by(|a, b| {
        if a > b { Ordering::Greater } else { Ordering::Less }
    })
}

fn generic_algorithms() {
    println!("\n=== 11. PRACTICAL GENERIC ALGORITHMS ===\n");

    let numbers = vec![1, 5, 3, 9, 2];
    println!("Finding 3 in {:?}: {:?}", numbers, find(&numbers, &3));

    let strings = vec!["apple", "banana", "cherry"];
    println!("Finding 'banana' in {:?}: {:?}", strings, find(&strings, &"banana"));

    println!("Max of {:?}: {:?}", numbers, find_max(&numbers));
}

// ============================================================================
// 12. TRAIT COMPOSITION
// ============================================================================

trait Readable {
    fn read(&self) -> String;
}

trait Writable {
    fn write(&mut self, content: String);
}

// Combine traits
trait ReadWrite: Readable + Writable {}

struct File {
    content: String,
}

impl Readable for File {
    fn read(&self) -> String {
        self.content.clone()
    }
}

impl Writable for File {
    fn write(&mut self, content: String) {
        self.content = content;
    }
}

impl ReadWrite for File {}

fn trait_composition() {
    println!("\n=== 12. TRAIT COMPOSITION ===\n");

    let mut file = File { content: "Hello".to_string() };
    println!("Reading: {}", file.read());
    
    file.write("Modified content".to_string());
    println!("After write: {}", file.read());
}

// ============================================================================
// MAIN
// ============================================================================
fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║   PART 8: TRAITS AND GENERICS        ║");
    println!("╚════════════════════════════════════════╝");

    traits_basics();
    trait_default_implementations();
    trait_objects();
    generics_basics();
    trait_bounds();
    associated_types();
    generic_multiple_types();
    generic_traits();
    trait_derivation();
    impl_trait();
    generic_algorithms();
    trait_composition();

    println!("\n╔════════════════════════════════════════╗");
    println!("║         ALL EXAMPLES COMPLETED        ║");
    println!("╚════════════════════════════════════════╝\n");
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. TRAITS:
//    - Define shared behavior across types
//    - Can have default implementations
//    - Enable static (compile-time) and dynamic (runtime) dispatch
//    - Similar to interfaces in other languages
//
// 2. TRAIT OBJECTS:
//    - Use &dyn Trait for dynamic dispatch
//    - Allows collections of different types
//    - Size is known at runtime, not compile time
//    - Slightly slower due to vtable lookups
//
// 3. GENERICS:
//    - Write code that works with multiple types
//    - Monomorphization: compiler creates separate code for each type
//    - No runtime overhead (unlike trait objects)
//    - Type checking happens at compile time
//
// 4. TRAIT BOUNDS:
//    - Constrain generic types to implement specific traits
//    - Can combine multiple traits with +
//    - Where clause provides cleaner syntax for complex bounds
//    - Enables writing flexible yet type-safe code
//
// 5. ASSOCIATED TYPES:
//    - Define types within traits
//    - Reduce number of type parameters
//    - Make trait implementations clearer
//    - Example: Iterator::Item
//
// 6. STATIC VS DYNAMIC DISPATCH:
//    - Generics: static dispatch (compile-time code generation)
//    - Trait objects: dynamic dispatch (runtime method lookup)
//    - Generics are faster but increase binary size
//    - Trait objects are flexible but slightly slower
//
// 7. IMPL TRAIT:
//    - Return trait-implementing types without naming them
//    - Uses static dispatch (compile-time)
//    - Only works as return type (not parameters)
//    - Useful for opaque types
//
// 8. BEST PRACTICES:
//    - Use traits to define behavior, not state
//    - Use generics for type-safe abstractions
//    - Use trait objects when you need heterogeneous collections
//    - Document trait bounds and requirements
//    - Prefer static dispatch (generics) for performance
// ============================================================================

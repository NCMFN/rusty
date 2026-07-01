// ============================================================================
// PART 9: LIFETIMES IN RUST
// ============================================================================
// Lifetimes track how long references are valid
// They are denoted with 'a, 'b, etc.
// Lifetimes are a compile-time feature - they don't exist at runtime
// The borrow checker uses lifetimes to prevent dangling references
// ============================================================================

use std::fmt;

// ============================================================================
// 1. BASIC LIFETIMES
// ============================================================================

// A reference with a lifetime
fn print_string<'a>(s: &'a str) {
    println!("String: {}", s);
}

// Function with reference parameters
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn basic_lifetimes() {
    println!("\n=== 1. BASIC LIFETIMES ===\n");

    let string1 = String::from("Hello");
    let string2 = "World";

    print_string(&string1);
    print_string(string2);

    let result = longest(&string1, string2);
    println!("Longest: {}", result);

    // This would fail at compile time:
    // let result;
    // {
    //     let string3 = String::from("temp");
    //     result = &string3; // borrow does not live long enough
    // }
    // println!("{}", result);
}

// ============================================================================
// 2. LIFETIME ANNOTATION RULES
// ============================================================================

// Rule 1: Each parameter gets its own lifetime
fn rule1<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

// Rule 2: If only one input lifetime, it's assigned to output
fn rule2(x: &str) -> &str {
    x
}

// Rule 3: If &self or &mut self, its lifetime is assigned to output
struct Book {
    title: String,
}

impl Book {
    fn get_title(&self) -> &str {
        &self.title
    }
}

fn lifetime_annotation_rules() {
    println!("\n=== 2. LIFETIME ANNOTATION RULES ===\n");

    let x = "Hello";
    let y = "World";
    println!("Rule 1 result: {}", rule1(x, y));

    println!("Rule 2 result: {}", rule2("Rust"));

    let book = Book { title: "The Rust Book".to_string() };
    println!("Book title: {}", book.get_title());
}

// ============================================================================
// 3. STRUCT WITH LIFETIME PARAMETERS
// ============================================================================

// A struct that holds a reference
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn structs_with_lifetimes() {
    println!("\n=== 3. STRUCT WITH LIFETIME PARAMETERS ===\n");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    
    let excerpt = Excerpt { part: first_sentence };
    println!("Excerpt: {}", excerpt.part);
    
    let part = excerpt.announce_and_return_part("Here's an important part:");
    println!("Returned part: {}", part);
}

// ============================================================================
// 4. MULTIPLE LIFETIME PARAMETERS
// ============================================================================

struct Person<'a> {
    name: &'a str,
}

struct Relationship<'a, 'b> {
    person1: &'a Person<'a>,
    person2: &'b Person<'b>,
}

impl<'a, 'b> Relationship<'a, 'b> {
    fn describe(&self) -> String {
        format!("{} and {}", self.person1.name, self.person2.name)
    }
}

fn multiple_lifetimes() {
    println!("\n=== 4. MULTIPLE LIFETIME PARAMETERS ===\n");

    let person1 = Person { name: "Alice" };
    let person2 = Person { name: "Bob" };
    
    let relationship = Relationship {
        person1: &person1,
        person2: &person2,
    };
    
    println!("Relationship: {}", relationship.describe());
}

// ============================================================================
// 5. LIFETIME ELISION
// ============================================================================

// These don't need explicit lifetime annotations because
// the compiler can infer them (lifetime elision rules)

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn last_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate().rev() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }
    &s[..]
}

fn lifetime_elision() {
    println!("\n=== 5. LIFETIME ELISION ===\n");

    let sentence = "Hello World Rust";
    println!("First word: {}", first_word(sentence));
    println!("Last word: {}", last_word(sentence));
}

// ============================================================================
// 6. STATIC LIFETIME
// ============================================================================

// 'static means the reference is valid for the entire program
const GREETING: &'static str = "Hello, world!";

fn static_lifetime() {
    println!("\n=== 6. STATIC LIFETIME ===\n");

    println!("Static string: {}", GREETING);

    let static_str: &'static str = "This is static";
    println!("Static reference: {}", static_str);

    // String literals have 'static lifetime
    let s: &'static str = "String literal";
    println!("String literal lifetime: {}", s);
}

// ============================================================================
// 7. LIFETIME BOUNDS
// ============================================================================

// Lifetime bounds specify that one lifetime must outlive another
fn compare<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    // 'b: 'a means 'b outlives 'a (b lives at least as long as a)
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Cache<'a> {
    data: &'a str,
}

impl<'a> Cache<'a> {
    fn new(data: &'a str) -> Self {
        Cache { data }
    }
}

fn lifetime_bounds() {
    println!("\n=== 7. LIFETIME BOUNDS ===\n");

    let s1 = String::from("Hello");
    let s2 = "World";
    
    let result = compare(&s1, s2);
    println!("Result: {}", result);

    let cache = Cache::new("cached data");
    println!("Cached: {}", cache.data);
}

// ============================================================================
// 8. TRAIT OBJECTS WITH LIFETIMES
// ============================================================================

trait Printable {
    fn print(&self);
}

struct Text<'a> {
    content: &'a str,
}

impl<'a> Printable for Text<'a> {
    fn print(&self) {
        println!("Text: {}", self.content);
    }
}

fn trait_objects_lifetimes() {
    println!("\n=== 8. TRAIT OBJECTS WITH LIFETIMES ===\n");

    let text = Text { content: "Hello Rust" };
    text.print();

    let printable: &dyn Printable = &text;
    printable.print();
}

// ============================================================================
// 9. LIFETIME VARIANCE
// ============================================================================

// Covariance: &'a T is covariant
// Contravariance: &'a mut T is contravariant
// Invariance: Cell<&'a T> is invariant

fn covariance<'a>(x: &'a str) -> &'a str {
    // A longer lived reference can be used where a shorter lived is expected
    x
}

fn lifetime_variance() {
    println!("\n=== 9. LIFETIME VARIANCE ===\n");

    let s = String::from("Hello");
    let shorter: &str = &s;
    let result = covariance(shorter);
    println!("Variance result: {}", result);
}

// ============================================================================
// 10. PRACTICAL EXAMPLE: PARSER
// ============================================================================

struct Parser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, position: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn next(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.position += 1;
        Some(ch)
    }

    fn parse_word(&mut self) -> &'a str {
        let start = self.position;
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() {
                self.next();
            } else {
                break;
            }
        }
        &self.input[start..self.position]
    }
}

fn practical_parser() {
    println!("\n=== 10. PRACTICAL EXAMPLE: PARSER ===\n");

    let mut parser = Parser::new("hello world rust");
    
    let word1 = parser.parse_word();
    println!("First word: {}", word1);
    
    parser.next(); // skip space
    
    let word2 = parser.parse_word();
    println!("Second word: {}", word2);
}

// ============================================================================
// 11. REFERENCES VS OWNED DATA
// ============================================================================

fn references_vs_owned() {
    println!("\n=== 11. REFERENCES VS OWNED DATA ===\n");

    // With owned data, no lifetime needed
    fn process_owned(s: String) -> String {
        println!("Processing: {}", s);
        s.to_uppercase()
    }

    // With references, lifetime must be specified
    fn process_borrowed<'a>(s: &'a str) -> &'a str {
        println!("Processing: {}", s);
        s
    }

    let owned = String::from("hello");
    let processed = process_owned(owned);
    println!("Result: {}", processed);

    let borrowed = "world";
    let result = process_borrowed(borrowed);
    println!("Result: {}", result);
}

// ============================================================================
// 12. COMMON LIFETIME MISTAKES
// ============================================================================

fn lifetime_mistakes_examples() {
    println!("\n=== 12. LIFETIME MISTAKES ===\n");

    // This compiles - 'a is inferred from the return type
    fn example1<'a>(x: &'a str) -> &'a str {
        x
    }

    // This is the same as above (lifetime elision)
    fn example2(x: &str) -> &str {
        x
    }

    // This would NOT compile - returning reference to local variable
    // fn bad_lifetime() -> &'static str {
    //     let s = String::from("hello");
    //     &s // error: cannot return reference to local variable
    // }

    println!("Lifetime examples compiled successfully");
    println!("example1 result: {}", example1("test"));
    println!("example2 result: {}", example2("test"));
}

// ============================================================================
// 13. ADVANCED: HIGHER-RANKED TRAIT BOUNDS (HRTB)
// ============================================================================

// for<'a> means: for all lifetimes 'a
trait HigherRanked {
    fn borrow(&self) -> &str;
}

fn accepts_hrtb<T: for<'a> Fn(&'a str) -> &'a str>(f: T) -> &'static str {
    "Function accepted"
}

fn advanced_hrtb() {
    println!("\n=== 13. HIGHER-RANKED TRAIT BOUNDS ===\n");

    // This closure works for any lifetime
    let f = |s: &str| s;
    let result = accepts_hrtb(f);
    println!("HRTB result: {}", result);
}

// ============================================================================
// 14. LIFETIME VISUALIZATION
// ============================================================================

fn lifetime_visualization() {
    println!("\n=== 14. LIFETIME VISUALIZATION ===\n");

    println!("Lifetime 'a spans from its declaration to end of scope");
    println!("References cannot outlive the data they reference");
    println!("");
    
    {
        let x = 5;           // Start of 'x lifetime
        let r = &x;          // Start of 'r lifetime (borrows 'x)
        println!("r = {}", r); // 'r is valid here
    } // End of both 'x and 'r lifetimes
    
    // println!("{}", r);   // ERROR: r would be out of scope
}

// ============================================================================
// MAIN
// ============================================================================
fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║      PART 9: LIFETIMES IN RUST       ║");
    println!("╚════════════════════════════════════════╝");

    basic_lifetimes();
    lifetime_annotation_rules();
    structs_with_lifetimes();
    multiple_lifetimes();
    lifetime_elision();
    static_lifetime();
    lifetime_bounds();
    trait_objects_lifetimes();
    lifetime_variance();
    practical_parser();
    references_vs_owned();
    lifetime_mistakes_examples();
    advanced_hrtb();
    lifetime_visualization();

    println!("\n╔════════════════════════════════════════╗");
    println!("║         ALL EXAMPLES COMPLETED        ║");
    println!("╚════════════════════════════════════════╝\n");
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. WHAT ARE LIFETIMES:
//    - Lifetimes are compile-time constructs that track reference validity
//    - They prevent dangling references and memory safety issues
//    - Denoted with 'a, 'b, 'lifetime, etc.
//    - Not present at runtime - only for compile-time checking
//
// 2. LIFETIME ANNOTATIONS:
//    - Required when functions/structs use references
//    - Parameters get lifetime parameter in angle brackets: <'a>
//    - References annotated: &'a T or &'a mut T
//    - Return types must have lifetime to show which input it ties to
//
// 3. LIFETIME ELISION RULES:
//    - Rule 1: Each parameter gets its own lifetime
//    - Rule 2: If one input lifetime, it's assigned to output
//    - Rule 3: If &self/&mut self, its lifetime goes to output
//    - Compiler applies these automatically
//
// 4. STRUCT LIFETIMES:
//    - Structs holding references need lifetime parameters
//    - Lifetime indicates how long the reference is valid
//    - Important for zero-copy data structures
//
// 5. STATIC LIFETIME:
//    - 'static means valid for entire program duration
//    - String literals and constants have 'static
//    - Can convert to 'static through Box::leak()
//
// 6. LIFETIME BOUNDS:
//    - 'b: 'a means 'b outlives 'a
//    - Constraints on how lifetimes relate to each other
//    - Essential for complex lifetime relationships
//
// 7. TRAIT OBJECTS WITH LIFETIMES:
//    - Trait objects can have lifetime parameters
//    - &'a dyn Trait + 'b is valid syntax
//    - Lifetimes follow same rules as other references
//
// 8. COMMON PATTERNS:
//    - &str for borrowed strings (lifetime inferred)
//    - Vec<&'a T> for collections of references
//    - &'a self for self-referential structures
//
// 9. WHEN TO USE WHAT:
//    - Use owned types (String, Vec) when possible
//    - Use references (&T) when borrowing
//    - Use 'static for compile-time known data
//    - Use lifetime parameters with structs containing refs
//
// 10. DEBUGGING LIFETIME ERRORS:
//    - Error messages point to specific lifetimes
//    - "borrowed value does not live long enough"
//    - "lifetime parameter not used"
//    - Trust the compiler - it's helping you write safe code!
//
// 11. VARIANCE:
//    - Covariance: longer lived ref can be used for shorter lived
//    - Contravariance: shorter lived ref can be used for longer lived
//    - Invariance: exact lifetime must match (rare)
//
// 12. BEST PRACTICES:
//    - Keep lifetimes as simple as possible
//    - Use lifetime elision when possible
//    - Document complex lifetime relationships
//    - Prefer owned types in public APIs
//    - Use references for performance and zero-copy
// ============================================================================

// ============================================================================
// PART 7: ERROR HANDLING IN RUST
// ============================================================================
// Rust has two main error handling mechanisms:
// 1. panic!() - Unrecoverable errors (aborts execution)
// 2. Result<T, E> - Recoverable errors (returns error for handling)
//
// Rust doesn't have exceptions like other languages.
// Instead, it forces you to handle errors explicitly at compile time.
// ============================================================================

use std::fs::File;
use std::io::{self, Read, Write};
use std::num::ParseIntError;

// ============================================================================
// 1. PANIC! - UNRECOVERABLE ERRORS
// ============================================================================
fn panic_examples() {
    println!("\n=== 1. PANIC! - UNRECOVERABLE ERRORS ===\n");

    // Explicit panic
    println!("About to panic...");
    // panic!("This is a panic message!");
    // Commented out so program continues

    // Array index out of bounds (panics)
    let arr = [1, 2, 3];
    println!("Accessing arr[1]: {}", arr[1]);
    // println!("Accessing arr[10]: {}", arr[10]); // Would panic

    // Division by zero (panics in debug mode)
    let x = 5;
    let y = 0;
    // let z = x / y; // Would panic in debug

    // Unwrap panics if None/Err
    let some_value = Some(5);
    println!("Some value unwrapped: {}", some_value.unwrap());

    // This would panic:
    let none_value: Option<i32> = None;
    // println!("{}", none_value.unwrap()); // Would panic

    println!("Panic examples completed (without actually panicking)");
}

// ============================================================================
// 2. RESULT<T, E> - RECOVERABLE ERRORS
// ============================================================================
fn result_basics() {
    println!("\n=== 2. RESULT<T, E> - RECOVERABLE ERRORS ===\n");

    // Result is an enum with two variants: Ok(T) or Err(E)
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // Pattern matching with Result
    let result: Result<i32, String> = Ok(42);
    
    match result {
        Ok(value) => println!("Success! Value: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // Err case
    let error_result: Result<i32, String> = Err("Something went wrong".to_string());
    
    match error_result {
        Ok(value) => println!("Value: {}", value),
        Err(e) => println!("Error occurred: {}", e),
    }

    // is_ok() and is_err()
    let ok_result: Result<i32, String> = Ok(5);
    println!("is_ok(): {}", ok_result.is_ok());
    println!("is_err(): {}", ok_result.is_err());

    // ok() and err() - convert Result to Option
    println!("ok(): {:?}", ok_result.ok());
    println!("err(): {:?}", ok_result.err());
}

// ============================================================================
// 3. UNWRAP AND EXPECT
// ============================================================================
fn unwrap_and_expect() {
    println!("\n=== 3. UNWRAP AND EXPECT ===\n");

    // unwrap() - panics if Err
    let result: Result<i32, &str> = Ok(10);
    let value = result.unwrap();
    println!("Unwrapped value: {}", value);

    // expect() - panics with custom message
    let result2: Result<i32, &str> = Ok(20);
    let value2 = result2.expect("Failed to unwrap result2");
    println!("Expected value: {}", value2);

    // unwrap_or() - default value if Err
    let result3: Result<i32, &str> = Err("error");
    let value3 = result3.unwrap_or(0);
    println!("Unwrap_or value: {}", value3);

    // unwrap_or_else() - closure if Err
    let result4: Result<i32, &str> = Err("error");
    let value4 = result4.unwrap_or_else(|_| 99);
    println!("Unwrap_or_else value: {}", value4);
}

// ============================================================================
// 4. THE ? OPERATOR (PROPAGATE ERRORS)
// ============================================================================
fn read_file_contents(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn question_mark_operator() {
    println!("\n=== 4. THE ? OPERATOR ===\n");

    // The ? operator:
    // - Returns the value if Ok
    // - Returns early with Err if Error
    // - Much cleaner than match statements

    // Try to read a file (will error if file doesn't exist)
    match read_file_contents("nonexistent.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }

    println!("The ? operator allows early return on error");
}

// ============================================================================
// 5. CUSTOM ERROR TYPES
// ============================================================================
#[derive(Debug)]
enum CalculationError {
    DivisionByZero,
    InvalidInput,
}

fn safe_divide(a: i32, b: i32) -> Result<i32, CalculationError> {
    if b == 0 {
        Err(CalculationError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn custom_error_types() {
    println!("\n=== 5. CUSTOM ERROR TYPES ===\n");

    match safe_divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }

    match safe_divide(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {:?}", e),
    }
}

// ============================================================================
// 6. CONVERTING BETWEEN RESULT AND OPTION
// ============================================================================
fn result_option_conversion() {
    println!("\n=== 6. RESULT TO OPTION CONVERSION ===\n");

    let ok_result: Result<i32, String> = Ok(42);
    
    // Result to Option
    let option = ok_result.ok();
    println!("Result to Option: {:?}", option);

    // Option to Result
    let option2: Option<i32> = Some(99);
    let result = option2.ok_or("Value was None");
    println!("Option to Result: {:?}", result);

    // and_then for chaining
    let x: Result<i32, &str> = Ok(2);
    let result = x.and_then(|n| {
        if n > 0 {
            Ok(n * 2)
        } else {
            Err("Must be positive")
        }
    });
    println!("and_then result: {:?}", result);
}

// ============================================================================
// 7. ERROR PROPAGATION PATTERNS
// ============================================================================
fn parse_pair(s: &str) -> Result<(i32, i32), ParseIntError> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        // This is a simplified example - in reality you'd want a custom error
        return Err("Invalid format".parse().unwrap_err());
    }
    
    let x = parts[0].trim().parse::<i32>()?;
    let y = parts[1].trim().parse::<i32>()?;
    Ok((x, y))
}

fn error_propagation() {
    println!("\n=== 7. ERROR PROPAGATION ===\n");

    match parse_pair("10,20") {
        Ok((x, y)) => println!("Parsed: ({}, {})", x, y),
        Err(e) => println!("Parse error: {}", e),
    }

    match parse_pair("10, hello") {
        Ok((x, y)) => println!("Parsed: ({}, {})", x, y),
        Err(e) => println!("Parse error: {}", e),
    }
}

// ============================================================================
// 8. RESULT WITH MAP AND AND_THEN
// ============================================================================
fn map_operations() {
    println!("\n=== 8. MAP AND AND_THEN ===\n");

    let x: Result<i32, &str> = Ok(5);
    
    // map() - transform Ok value
    let y = x.map(|n| n * 2);
    println!("After map(|n| n * 2): {:?}", y);

    // map_err() - transform Err value
    let z: Result<i32, &str> = Err("error");
    let z2 = z.map_err(|e| format!("Error: {}", e));
    println!("After map_err: {:?}", z2);

    // and_then() - chain operations that return Result
    let result = Ok(2)
        .and_then(|n| Ok(n * 3))
        .and_then(|n| Ok(n + 1));
    println!("Chained and_then: {:?}", result);

    // or_else() - provide alternative on Err
    let result2: Result<i32, &str> = Err("error");
    let result3 = result2.or_else(|_| Ok(99));
    println!("or_else result: {:?}", result3);
}

// ============================================================================
// 9. PRACTICAL ERROR HANDLING EXAMPLES
// ============================================================================
fn validate_email(email: &str) -> Result<String, String> {
    if email.is_empty() {
        return Err("Email cannot be empty".to_string());
    }
    
    if !email.contains('@') {
        return Err("Email must contain @".to_string());
    }
    
    if !email.contains('.') {
        return Err("Email must contain domain".to_string());
    }
    
    Ok(email.to_string())
}

fn parse_age(s: &str) -> Result<u32, String> {
    let age = s.parse::<u32>()
        .map_err(|_| "Age must be a valid number".to_string())?;
    
    if age < 0 || age > 150 {
        return Err("Age must be between 0 and 150".to_string());
    }
    
    Ok(age)
}

struct User {
    email: String,
    age: u32,
}

fn create_user(email_str: &str, age_str: &str) -> Result<User, String> {
    let email = validate_email(email_str)?;
    let age = parse_age(age_str)?;
    
    Ok(User { email, age })
}

fn practical_examples() {
    println!("\n=== 9. PRACTICAL EXAMPLES ===\n");

    // Valid user
    match create_user("alice@example.com", "25") {
        Ok(user) => println!("User created: {} (age {})", user.email, user.age),
        Err(e) => println!("Failed to create user: {}", e),
    }

    // Invalid email
    match create_user("invalid-email", "25") {
        Ok(user) => println!("User created: {} (age {})", user.email, user.age),
        Err(e) => println!("Failed to create user: {}", e),
    }

    // Invalid age
    match create_user("bob@example.com", "200") {
        Ok(user) => println!("User created: {} (age {})", user.email, user.age),
        Err(e) => println!("Failed to create user: {}", e),
    }
}

// ============================================================================
// 10. RESULT ITERATOR METHODS
// ============================================================================
fn result_iterators() {
    println!("\n=== 10. RESULT WITH ITERATORS ===\n");

    let numbers = vec!["1", "2", "three", "4"];
    
    // Collect results
    let results: Result<Vec<i32>, _> = numbers
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();
    
    match results {
        Ok(nums) => println!("Parsed all: {:?}", nums),
        Err(e) => println!("Parse failed: {}", e),
    }

    // Continue on error with collect()
    let partial: Vec<i32> = vec!["1", "2", "three", "4"]
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Parsed valid numbers: {:?}", partial);
}

// ============================================================================
// MAIN
// ============================================================================
fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║    PART 7: ERROR HANDLING IN RUST     ║");
    println!("╚════════════════════════════════════════╝");

    panic_examples();
    result_basics();
    unwrap_and_expect();
    question_mark_operator();
    custom_error_types();
    result_option_conversion();
    error_propagation();
    map_operations();
    practical_examples();
    result_iterators();

    println!("\n╔════════════════════════════════════════╗");
    println!("║         ALL EXAMPLES COMPLETED        ║");
    println!("╚════════════════════════════════════════╝\n");
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Panic vs Result:
//    - Use panic!() only for truly unrecoverable errors
//    - Use Result<T, E> for errors that can be handled
//
// 2. Pattern Matching:
//    - Match on Result variants explicitly
//    - Compiler forces you to handle all cases
//
// 3. The ? Operator:
//    - Shorthand for error propagation
//    - Only works in functions returning Result
//    - Makes code cleaner and more readable
//
// 4. Unwrap Methods:
//    - unwrap() - panic if Err
//    - expect() - panic with custom message
//    - unwrap_or() - provide default value
//    - unwrap_or_else() - compute default with closure
//
// 5. Transformations:
//    - map() - transform Ok value
//    - map_err() - transform Err value
//    - and_then() - chain operations returning Result
//    - or_else() - provide alternative on Err
//
// 6. Best Practices:
//    - Create custom error types for your domain
//    - Propagate errors up the call stack
//    - Don't unwrap in library code
//    - Document which errors can be returned
//    - Use descriptive error messages
// ============================================================================

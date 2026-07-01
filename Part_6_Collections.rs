// ============================================================================
// PART 6: COLLECTIONS IN RUST
// ============================================================================
// Rust provides several collection types in the standard library:
// - Vec<T>: Growable array (most common)
// - HashMap<K, V>: Key-value store (unordered)
// - HashSet<T>: Unique values (unordered)
// - VecDeque<T>: Double-ended queue
// - BTreeMap<K, V>: Sorted key-value store
// - BTreeSet<T>: Sorted unique values
// ============================================================================

// ============================================================================
// 1. VECTORS (Vec<T>)
// ============================================================================
fn vectors_example() {
    println!("\n=== 1. VECTORS ===\n");

    // Creating vectors
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Vector: {:?}", v);

    // Using vec! macro
    let v2 = vec![1, 2, 3, 4, 5];
    println!("Vec macro: {:?}", v2);

    // Accessing elements
    println!("First element: {}", v2[0]);
    println!("Using get(): {:?}", v2.get(1));
    println!("Using get() out of bounds: {:?}", v2.get(100));

    // Iterating
    println!("Iterating:");
    for (i, val) in v2.iter().enumerate() {
        println!("  v2[{}] = {}", i, val);
    }

    // Mutable iteration
    let mut v3 = vec![1, 2, 3];
    for val in &mut v3 {
        *val *= 2;
    }
    println!("After doubling: {:?}", v3);

    // Vector operations
    let v4 = vec![10, 20, 30];
    println!("Length: {}", v4.len());
    println!("Capacity: {}", v4.capacity());
    println!("Is empty: {}", v4.is_empty());

    // Pop and remove
    let mut v5 = vec![1, 2, 3, 4, 5];
    println!("Popped: {:?}", v5.pop());
    println!("After pop: {:?}", v5);
    
    v5.remove(1); // Remove at index 1
    println!("After removing index 1: {:?}", v5);

    // Collecting into vector
    let v6: Vec<i32> = (1..=5).collect();
    println!("Collected range: {:?}", v6);

    // Filtering and mapping
    let v7 = vec![1, 2, 3, 4, 5, 6];
    let evens: Vec<i32> = v7.iter().filter(|&&x| x % 2 == 0).copied().collect();
    println!("Even numbers: {:?}", evens);

    let doubled: Vec<i32> = v7.iter().map(|&x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
}

// ============================================================================
// 2. HASH MAPS (HashMap<K, V>)
// ============================================================================
fn hash_maps_example() {
    use std::collections::HashMap;
    
    println!("\n=== 2. HASH MAPS ===\n");

    // Creating a HashMap
    let mut scores = HashMap::new();
    scores.insert("Alice", 90);
    scores.insert("Bob", 85);
    scores.insert("Charlie", 92);

    println!("Scores: {:?}", scores);

    // Accessing values
    match scores.get("Alice") {
        Some(score) => println!("Alice's score: {}", score),
        None => println!("Alice not found"),
    }

    // Using entry API for conditional insertion
    scores.entry("David").or_insert(88);
    scores.entry("Alice").or_insert(999); // Won't overwrite
    println!("After entry: {:?}", scores);

    // Iterating
    println!("All scores:");
    for (name, score) in &scores {
        println!("  {}: {}", name, score);
    }

    // Updating values
    scores.insert("Bob", 95); // Update
    println!("Updated Bob: {:?}", scores);

    // Removing entries
    scores.remove("Charlie");
    println!("After removing Charlie: {:?}", scores);

    // Counting occurrences
    let text = "hello world hello rust hello";
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut count = HashMap::new();
    
    for word in words {
        *count.entry(word).or_insert(0) += 1;
    }
    println!("Word count: {:?}", count);

    // HashMap with owned types
    let mut inventory = HashMap::new();
    inventory.insert(String::from("apple"), 5);
    inventory.insert(String::from("banana"), 3);
    println!("Inventory: {:?}", inventory);
}

// ============================================================================
// 3. HASH SETS (HashSet<T>)
// ============================================================================
fn hash_sets_example() {
    use std::collections::HashSet;
    
    println!("\n=== 3. HASH SETS ===\n");

    // Creating a HashSet
    let mut numbers = HashSet::new();
    numbers.insert(1);
    numbers.insert(2);
    numbers.insert(3);
    numbers.insert(2); // Duplicate, won't be added
    
    println!("Set: {:?}", numbers);
    println!("Length: {}", numbers.len());

    // Using from_iter
    let set1: HashSet<i32> = vec![1, 2, 3, 4].into_iter().collect();
    let set2: HashSet<i32> = vec![3, 4, 5, 6].into_iter().collect();

    // Set operations
    println!("Set 1: {:?}", set1);
    println!("Set 2: {:?}", set2);

    let union: HashSet<_> = set1.union(&set2).copied().collect();
    println!("Union: {:?}", union);

    let intersection: HashSet<_> = set1.intersection(&set2).copied().collect();
    println!("Intersection: {:?}", intersection);

    let difference: HashSet<_> = set1.difference(&set2).copied().collect();
    println!("Difference (1 - 2): {:?}", difference);

    let symmetric_diff: HashSet<_> = set1.symmetric_difference(&set2).copied().collect();
    println!("Symmetric difference: {:?}", symmetric_diff);

    // Checking membership
    println!("Contains 2: {}", set1.contains(&2));
    println!("Contains 10: {}", set1.contains(&10));

    // Removing elements
    let mut set3 = HashSet::from([1, 2, 3, 4, 5]);
    set3.remove(&3);
    println!("After remove(3): {:?}", set3);
}

// ============================================================================
// 4. VEC DEQUE (Double-ended Queue)
// ============================================================================
fn vec_deque_example() {
    use std::collections::VecDeque;
    
    println!("\n=== 4. VEC DEQUE ===\n");

    let mut deque = VecDeque::new();
    
    // Adding to front and back
    deque.push_back(1);
    deque.push_back(2);
    deque.push_back(3);
    deque.push_front(0);
    
    println!("Deque: {:?}", deque);

    // Removing from front and back
    println!("Pop front: {:?}", deque.pop_front());
    println!("Pop back: {:?}", deque.pop_back());
    println!("After pops: {:?}", deque);

    // Accessing
    println!("Front: {:?}", deque.front());
    println!("Back: {:?}", deque.back());

    // Useful for queues and BFS
    let mut queue = VecDeque::new();
    for i in 1..=3 {
        queue.push_back(i);
    }
    
    while let Some(item) = queue.pop_front() {
        println!("Processing: {}", item);
    }
}

// ============================================================================
// 5. BTREE MAP (Sorted Key-Value Store)
// ============================================================================
fn btree_map_example() {
    use std::collections::BTreeMap;
    
    println!("\n=== 5. BTREE MAP ===\n");

    let mut map = BTreeMap::new();
    map.insert(3, "three");
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(5, "five");

    // Keys are sorted
    println!("BTreeMap (sorted): {:?}", map);

    // Range queries
    let range: Vec<_> = map.range(1..=3).collect();
    println!("Range [1..=3]: {:?}", range);

    // First and last
    println!("First: {:?}", map.first_key_value());
    println!("Last: {:?}", map.last_key_value());

    // Iteration in order
    println!("In order:");
    for (k, v) in &map {
        println!("  {} => {}", k, v);
    }
}

// ============================================================================
// 6. BTREE SET (Sorted Set)
// ============================================================================
fn btree_set_example() {
    use std::collections::BTreeSet;
    
    println!("\n=== 6. BTREE SET ===\n");

    let mut set = BTreeSet::new();
    set.insert(5);
    set.insert(1);
    set.insert(3);
    set.insert(2);

    // Elements are sorted
    println!("BTreeSet (sorted): {:?}", set);

    // Range operations
    let range: Vec<_> = set.range(2..=4).collect();
    println!("Range [2..=4]: {:?}", range);

    // Lowest and highest
    println!("First: {:?}", set.first());
    println!("Last: {:?}", set.last());
}

// ============================================================================
// 7. PRACTICAL EXAMPLES
// ============================================================================
fn practical_examples() {
    use std::collections::{HashMap, HashSet};
    
    println!("\n=== 7. PRACTICAL EXAMPLES ===\n");

    // Example 1: Unique words in a sentence
    println!("--- Unique Words ---");
    let text = "the quick brown fox jumps over the lazy dog";
    let unique_words: HashSet<&str> = text.split_whitespace().collect();
    println!("Unique words: {:?}", unique_words);
    println!("Count: {}", unique_words.len());

    // Example 2: Grade distribution
    println!("\n--- Grade Distribution ---");
    let grades = vec![85, 92, 78, 92, 85, 88, 92, 78];
    let mut distribution: HashMap<i32, i32> = HashMap::new();
    
    for grade in grades {
        *distribution.entry(grade).or_insert(0) += 1;
    }
    
    println!("Grade distribution:");
    for (grade, count) in &distribution {
        println!("  Grade {}: {} students", grade, count);
    }

    // Example 3: Simple cache
    println!("\n--- Simple Cache ---");
    let mut cache: HashMap<String, String> = HashMap::new();
    
    let data = vec![
        ("user1".to_string(), "Alice".to_string()),
        ("user2".to_string(), "Bob".to_string()),
        ("user1".to_string(), "Alice Updated".to_string()),
    ];
    
    for (key, value) in data {
        cache.insert(key.clone(), value);
        println!("Cached {} => {}", key, cache.get(&key).unwrap());
    }

    // Example 4: Two-sum problem
    println!("\n--- Two-Sum Problem ---");
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    
    let mut seen = HashSet::new();
    for &num in &nums {
        let complement = target - num;
        if seen.contains(&complement) {
            println!("Found pair: {} + {} = {}", complement, num, target);
            break;
        }
        seen.insert(num);
    }
}

// ============================================================================
// MAIN
// ============================================================================
fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║      PART 6: RUST COLLECTIONS        ║");
    println!("╚════════════════════════════════════════╝");

    vectors_example();
    hash_maps_example();
    hash_sets_example();
    vec_deque_example();
    btree_map_example();
    btree_set_example();
    practical_examples();

    println!("\n╔════════════════════════════════════════╗");
    println!("║         ALL EXAMPLES COMPLETED        ║");
    println!("╚════════════════════════════════════════╝\n");
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Vectors (Vec<T>):
//    - Use for ordered, growable collections
//    - Fast indexed access O(1)
//    - Most commonly used collection
//
// 2. HashMap<K, V>:
//    - Use for key-value lookups
//    - Fast O(1) average lookup time
//    - Unordered (use BTreeMap if you need order)
//
// 3. HashSet<T>:
//    - Use for unique values and membership testing
//    - Fast O(1) average lookup
//    - Great for deduplication
//
// 4. VecDeque<T>:
//    - Use for queue/deque operations
//    - Efficient push/pop at both ends
//    - Good for BFS and queues
//
// 5. BTreeMap<K, V> & BTreeSet<T>:
//    - Use when you need sorted data
//    - O(log n) operations instead of O(1)
//    - Range queries are efficient
//
// 6. Performance Considerations:
//    - Choose based on your access patterns
//    - Hash collections are faster for lookups
//    - BTree collections maintain order
//    - Consider memory layout and cache locality
// ============================================================================

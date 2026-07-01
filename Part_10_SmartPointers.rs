// ============================================================================
// PART 10: SMART POINTERS IN RUST
// ============================================================================
// Smart pointers are data structures that act like pointers but have
// additional metadata and capabilities. They manage memory automatically
// through RAII (Resource Acquisition Is Initialization) pattern.
//
// Common smart pointers:
// - Box<T>: Heap allocation
// - Rc<T>: Reference counting (single-threaded shared ownership)
// - Arc<T>: Atomic reference counting (thread-safe shared ownership)
// - RefCell<T>: Interior mutability at runtime
// - Mutex<T>: Thread-safe interior mutability
// ============================================================================

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;
use std::fmt;

// ============================================================================
// 1. BOX<T> - HEAP ALLOCATION
// ============================================================================

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn box_basics() {
    println!("\n=== 1. BOX<T> - HEAP ALLOCATION ===\n");

    // Box allows us to store data on the heap
    let b = Box::new(5);
    println!("Boxed value: {}", b);

    // Using Box with recursive data structures
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List: {:?}", list);

    // Box takes ownership
    let x = Box::new("Hello");
    println!("Boxed string: {}", x);
    // x is dropped here, memory is freed

    // Box implements Deref and DerefMut
    let mut boxed = Box::new(42);
    *boxed = 100;
    println!("Modified boxed value: {}", boxed);
}

// ============================================================================
// 2. DEREF TRAIT
// ============================================================================

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn deref_trait() {
    println!("\n=== 2. DEREF TRAIT ===\n");

    let x = MyBox::new(5);
    println!("MyBox value: {}", *x);

    let mut y = MyBox::new(String::from("Hello"));
    println!("Original: {}", *y);
    
    y.0.push_str(" World");
    println!("After modification: {}", *y);

    // Deref coercion
    fn print_string(s: &str) {
        println!("String: {}", s);
    }

    let string_box = MyBox::new(String::from("Deref coercion"));
    print_string(&string_box); // Automatically deref'd
}

// ============================================================================
// 3. DROP TRAIT
// ============================================================================

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}

fn drop_trait() {
    println!("\n=== 3. DROP TRAIT ===\n");

    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    
    println!("CustomSmartPointer created");
    
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    
    println!("CustomSmartPointers about to go out of scope");
    // _d and _c are dropped in reverse order here
}

// ============================================================================
// 4. RC<T> - REFERENCE COUNTING (SINGLE-THREADED)
// ============================================================================

#[derive(Debug)]
enum RcList {
    RcCons(i32, Rc<RefCell<RcList>>),
    RcNil,
}

use RcList::{RcCons, RcNil};

fn rc_basics() {
    println!("\n=== 4. RC<T> - REFERENCE COUNTING ===\n");

    let a = Rc::new(RcCons(5, Rc::new(RefCell::new(RcNil))));
    println!("Reference count after creating a: {}", Rc::strong_count(&a));

    let b = RcCons(10, Rc::clone(&a));
    println!("Reference count after cloning for b: {}", Rc::strong_count(&a));

    {
        let _c = RcCons(20, Rc::clone(&a));
        println!("Reference count after cloning for c: {}", Rc::strong_count(&a));
    }
    
    println!("Reference count after c goes out of scope: {}", Rc::strong_count(&a));
}

// ============================================================================
// 5. REFCELL<T> - INTERIOR MUTABILITY
// ============================================================================

struct Book {
    title: String,
    borrowed_count: RefCell<u32>,
}

impl Book {
    fn new(title: &str) -> Book {
        Book {
            title: title.to_string(),
            borrowed_count: RefCell::new(0),
        }
    }

    fn borrow(&self) {
        *self.borrowed_count.borrow_mut() += 1;
        println!("{} borrowed (count: {})", self.title, self.borrowed_count.borrow());
    }

    fn return_book(&self) {
        *self.borrowed_count.borrow_mut() -= 1;
        println!("{} returned (count: {})", self.title, self.borrowed_count.borrow());
    }

    fn get_borrow_count(&self) -> u32 {
        *self.borrowed_count.borrow()
    }
}

fn refcell_basics() {
    println!("\n=== 5. REFCELL<T> - INTERIOR MUTABILITY ===\n");

    let book = Book::new("The Rust Book");
    
    book.borrow();
    book.borrow();
    println!("Current borrow count: {}", book.get_borrow_count());
    
    book.return_book();
    println!("After return: {}", book.get_borrow_count());

    // RefCell runtime borrow checking
    let value = RefCell::new(5);
    
    let borrow1 = value.borrow();
    println!("First borrow: {}", *borrow1);
    
    let borrow2 = value.borrow();
    println!("Second borrow: {}", *borrow2);
    // Multiple immutable borrows are OK
}

// ============================================================================
// 6. COMBINING RC<T> AND REFCELL<T>
// ============================================================================

#[derive(Debug)]
struct Node {
    value: i32,
    next: RefCell<Option<Rc<Node>>>,
}

fn rc_refcell_combination() {
    println!("\n=== 6. RC<T> + REFCELL<T> COMBINATION ===\n");

    let node1 = Rc::new(Node {
        value: 1,
        next: RefCell::new(None),
    });

    let node2 = Rc::new(Node {
        value: 2,
        next: RefCell::new(Some(Rc::clone(&node1))),
    });

    let node3 = Rc::new(Node {
        value: 3,
        next: RefCell::new(Some(Rc::clone(&node2))),
    });

    println!("Created linked list with Rc + RefCell");
    println!("Node3 value: {}", node3.value);
}

// ============================================================================
// 7. ARC<T> - THREAD-SAFE REFERENCE COUNTING
// ============================================================================

fn arc_basics() {
    println!("\n=== 7. ARC<T> - THREAD-SAFE REFERENCE COUNTING ===\n");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..3 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented counter", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}

// ============================================================================
// 8. MUTEX<T> - THREAD-SAFE INTERIOR MUTABILITY
// ============================================================================

fn mutex_basics() {
    println!("\n=== 8. MUTEX<T> - THREAD-SAFE INTERIOR MUTABILITY ===\n");

    let data = Mutex::new(vec![1, 2, 3]);

    {
        let mut vec = data.lock().unwrap();
        vec.push(4);
        println!("After adding 4: {:?}", *vec);
    } // Lock is released here

    {
        let vec = data.lock().unwrap();
        println!("Final vector: {:?}", *vec);
    }

    // Accessing from multiple threads
    let shared_data = Arc::new(Mutex::new(String::from("shared")));
    
    let data1 = Arc::clone(&shared_data);
    let handle1 = thread::spawn(move || {
        let mut s = data1.lock().unwrap();
        s.push_str(" from thread 1");
    });

    let data2 = Arc::clone(&shared_data);
    let handle2 = thread::spawn(move || {
        let mut s = data2.lock().unwrap();
        s.push_str(" from thread 2");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Shared string: {}", *shared_data.lock().unwrap());
}

// ============================================================================
// 9. WEAK<T> - PREVENTING REFERENCE CYCLES
// ============================================================================

use std::rc::Weak;

#[derive(Debug)]
struct TreeNode {
    value: i32,
    children: RefCell<Vec<Rc<TreeNode>>>,
    parent: RefCell<Weak<TreeNode>>,
}

fn weak_pointers() {
    println!("\n=== 9. WEAK<T> - PREVENTING REFERENCE CYCLES ===\n");

    let parent = Rc::new(TreeNode {
        value: 1,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    let child = Rc::new(TreeNode {
        value: 2,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::clone(&Rc::downgrade(&parent))),
    });

    parent.children.borrow_mut().push(Rc::clone(&child));

    println!("Parent: {}", parent.value);
    println!("Child: {}", child.value);
    
    if let Some(p) = child.parent.borrow().upgrade() {
        println!("Child's parent value: {}", p.value);
    }
}

// ============================================================================
// 10. COMPARISON OF SMART POINTERS
// ============================================================================

fn smart_pointer_comparison() {
    println!("\n=== 10. SMART POINTER COMPARISON ===\n");

    println!("Box<T>:");
    println!("  - Single ownership");
    println!("  - Heap allocation");
    println!("  - No runtime overhead");
    println!("  - Best for: recursive types, large data");

    println!("\nRc<T>:");
    println!("  - Multiple ownership (single-threaded)");
    println!("  - Heap allocation");
    println!("  - Reference counting overhead");
    println!("  - Best for: shared data, graphs, trees");

    println!("\nRefCell<T>:");
    println!("  - Interior mutability");
    println!("  - Runtime borrow checking");
    println!("  - Panic if multiple mutable borrows");
    println!("  - Best for: shared mutable data");

    println!("\nArc<T>:");
    println!("  - Thread-safe reference counting");
    println!("  - Heap allocation");
    println!("  - Atomic operations overhead");
    println!("  - Best for: shared data between threads");

    println!("\nMutex<T>:");
    println!("  - Thread-safe interior mutability");
    println!("  - Lock mechanism");
    println!("  - Potential for deadlocks");
    println!("  - Best for: thread-safe mutable data");
}

// ============================================================================
// 11. PRACTICAL EXAMPLE: GRAPH WITH SHARED NODES
// ============================================================================

#[derive(Debug)]
struct GraphNode {
    id: u32,
    neighbors: RefCell<Vec<Rc<GraphNode>>>,
}

impl GraphNode {
    fn new(id: u32) -> Self {
        GraphNode {
            id,
            neighbors: RefCell::new(vec![]),
        }
    }

    fn add_neighbor(&self, neighbor: Rc<GraphNode>) {
        self.neighbors.borrow_mut().push(neighbor);
    }

    fn get_neighbor_ids(&self) -> Vec<u32> {
        self.neighbors.borrow().iter().map(|n| n.id).collect()
    }
}

fn practical_graph() {
    println!("\n=== 11. PRACTICAL EXAMPLE: GRAPH ===\n");

    let node1 = Rc::new(GraphNode::new(1));
    let node2 = Rc::new(GraphNode::new(2));
    let node3 = Rc::new(GraphNode::new(3));

    node1.add_neighbor(Rc::clone(&node2));
    node1.add_neighbor(Rc::clone(&node3));
    node2.add_neighbor(Rc::clone(&node3));

    println!("Node 1 neighbors: {:?}", node1.get_neighbor_ids());
    println!("Node 2 neighbors: {:?}", node2.get_neighbor_ids());
    println!("Node 3 neighbors: {:?}", node3.get_neighbor_ids());
}

// ============================================================================
// 12. PRACTICAL EXAMPLE: THREAD-SAFE CACHE
// ============================================================================

struct Cache {
    data: Arc<Mutex<Vec<String>>>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            data: Arc::new(Mutex::new(vec![])),
        }
    }

    fn add(&self, item: String) {
        let mut cache = self.data.lock().unwrap();
        cache.push(item);
    }

    fn get_all(&self) -> Vec<String> {
        let cache = self.data.lock().unwrap();
        cache.clone()
    }
}

impl Clone for Cache {
    fn clone(&self) -> Self {
        Cache {
            data: Arc::clone(&self.data),
        }
    }
}

fn practical_thread_safe_cache() {
    println!("\n=== 12. PRACTICAL EXAMPLE: THREAD-SAFE CACHE ===\n");

    let cache = Cache::new();
    let mut handles = vec![];

    for i in 0..3 {
        let cache_clone = cache.clone();
        let handle = thread::spawn(move || {
            cache_clone.add(format!("Item from thread {}", i));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Cache contents: {:?}", cache.get_all());
}

// ============================================================================
// MAIN
// ============================================================================
fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║    PART 10: SMART POINTERS IN RUST   ║");
    println!("╚════════════════════════════════════════╝");

    box_basics();
    deref_trait();
    drop_trait();
    rc_basics();
    refcell_basics();
    rc_refcell_combination();
    arc_basics();
    mutex_basics();
    weak_pointers();
    smart_pointer_comparison();
    practical_graph();
    practical_thread_safe_cache();

    println!("\n╔════════════════════════════════════════╗");
    println!("║         ALL EXAMPLES COMPLETED        ║");
    println!("╚════════════════════════════════════════╝\n");
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. BOX<T>:
//    - Allocates on heap
//    - Single ownership
//    - Enables recursive types
//    - Zero runtime overhead
//
// 2. DEREF AND DROP TRAITS:
//    - Deref enables dereferencing: *boxed_value
//    - Deref coercion automatically dereferences
//    - Drop trait called when value dropped
//    - Enables RAII pattern
//
// 3. RC<T> (Reference Counting):
//    - Multiple ownership (single-threaded)
//    - Rc::clone() increases reference count
//    - Value dropped when last Rc is dropped
//    - Runtime reference count overhead
//    - NOT thread-safe
//
// 4. REFCELL<T> (Interior Mutability):
//    - Mutate through immutable reference
//    - Borrow checking happens at runtime
//    - Panics on multiple mutable borrows
//    - Single-threaded only
//
// 5. RC<T> + REFCELL<T>:
//    - Shared mutable data (single-threaded)
//    - Common pattern for complex data structures
//    - Enables shared mutable graphs and trees
//
// 6. ARC<T> (Atomic Reference Counting):
//    - Thread-safe version of Rc<T>
//    - Atomic operations are slower
//    - Use only when sharing between threads
//    - NOT mutable by itself
//
// 7. MUTEX<T>:
//    - Thread-safe interior mutability
//    - Lock must be held during mutation
//    - Potential for deadlocks if not careful
//    - Should be used with Arc<T>
//
// 8. WEAK<T>:
//    - Prevents reference cycles
//    - Doesn't prevent value from being dropped
//    - upgrade() returns Option<Rc<T>>
//    - Essential for parent-child relationships
//
// 9. SMART POINTER CHOICE:
//    - Box: Single ownership, heap allocation
//    - Rc: Shared ownership (single-threaded)
//    - Arc: Shared ownership (thread-safe)
//    - RefCell: Runtime borrow checking
//    - Mutex: Thread-safe mutation
//
// 10. MEMORY MANAGEMENT:
//    - Smart pointers handle cleanup automatically
//    - Prevents memory leaks (mostly)
//    - Weak pointers prevent reference cycles
//    - Use Valgrind or Miri to detect issues
//
// 11. PERFORMANCE:
//    - Box: No overhead
//    - Rc: Reference count checks
//    - Arc: Atomic operations slower than Rc
//    - RefCell: Runtime borrow checks
//    - Mutex: Lock operations can be expensive
//
// 12. BEST PRACTICES:
//    - Use Box for owned data by default
//    - Minimize use of Rc and Arc
//    - Avoid reference cycles (use Weak)
//    - Be cautious with Mutex deadlocks
//    - Document when using interior mutability
// ============================================================================

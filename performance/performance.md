---
title: Performance Optmizations in Rust.
---

Optimize what matters:
====

<!-- alignment: center -->

Performance Optimizations requires efforts and times, therefore it's crucial to optimize the right code blocks.

<!--pause-->

<!-- new_line -->

# Example

<!-- new_lines: 2 -->

```rust
strcut Proc{}

impl Proc {
  fn create(config_path: &Path) -> Self {...}
  fn process(&self, data: &[u8]) -> Vec<String> {...}
}

fn run(config_path: &Path) {
  let process = Proc::create(config_path);

  while let Some(data) = get_data() {
    let items = process.process(data);
    // Continue work on items.
  }
}
```
<!--pause-->

<!-- new_lines: 2 -->

* Function `process()` is called multiple times while function `create()` is called once.
* Optimizations of `process()` will benefit the overall performance x times.

<!-- end_slide -->

Profilers:
====

<!-- alignment: center -->

Profilers help us analyze application performance:

<!-- new_lines: 1 -->
# CPU Profilers
<!-- new_lines: 1 -->
* CPU Profilers identify where time is spent.
* Most famous profiler is `perf` on Linux, which has many tools built on the top of it to visualize its output like:
  - [Flamegraph](https://github.com/brendangregg/FlameGraph)
  - [Flamegraph-rs](https://github.com/flamegraph-rs/flamegraph)
  - [Samply](https://github.com/mstange/samply)

<!-- new_lines: 1 -->
# Samply Show Case:

<!--pause-->

<!-- jump_to_middle -->
# Memory Profilers:
<!-- new_lines: 1 -->
* Memory Profilers track memory allocations and usage patterns.
  - [DHAT](https://crates.io/crates/dhat) Experimental heap profiling in rust 

<!-- end_slide -->

Benchmarking
====
<!-- alignment: center -->

# Always Measure!

<!-- new_lines: 2 -->

Effective performance optimization cannot be based on intuition. The journey from source code to executable binary is layered with transformations—from the Rust compiler's initial optimizations, to deeper passes in the LLVM backend, and finally to platform-specific machine code. Even then, the CPU itself performs runtime optimizations like branch prediction and speculative execution.

Since no one can fully predict the outcome of this complex chain, our assumptions about 'slow code' are often wrong. Therefore, we must use profilers to measure and confirm where the real bottlenecks are before making any changes.

<!--pause-->

<!-- jump_to_middle -->
# Benchmarking Tools:

<!-- new_lines: 1 -->
* [Criterion](https://github.com/bheisler/criterion.rs): For benchmarking specific functions in code.
* [Hyperfine](https://github.com/sharkdp/hyperfine) For CLIs benchmarking.


<!-- end_slide -->

Compiler Optimizations
====
<!-- alignment: center -->

# Give the compiler all available infos:

The Rust compiler is very good at optimizing at compile time. As a general rule, the more information it has up front, the better its optimizations will be. When we write code that lets the compiler figure things out early, for example by using generics, we give it the chance to create faster and more specific machine code for each exact situation.

<!--pause-->

<!-- new_lines: 2 -->

# Example with Infallible errors:

<!-- new_lines: 1 -->

```rust
struct MyType {
    content: String,
}

impl FromStr for MyType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            content: s.to_owned(),
        })
    }
}

fn to_mytype(text: &str) -> MyType {
    // The whole match statement will be removed here because the compiler knows
    // That parsing into `MyType` is Infallible.
    match text.parse::<MyType>() {
        Ok(my) => my,
        Err(_err) => {
            // This will removed completely at compile time.
            unreachable!("MyType can be always generated from string")
        }
    }
}
```

<!-- end_slide -->

Compiler Optimizations
====

<!-- alignment: center -->

# Avoid Unreadable Code on Mathematical Expressions:
<!-- new_lines: 1 -->

In the past, developers often wrote unreadable code—like manual bit-shifts—to squeeze maximum performance out of mathematical expressions. Today, modern compilers like Rust's automatically perform these same optimizations on simple, readable code. Therefore, we should avoid these outdated 'clever' tricks and prioritize clarity, knowing that performance will not be sacrificed.

<!--pause-->

<!-- new_lines: 2 -->

# Example with is_odd:
<!-- new_lines: 1 -->

Rust compiler will producer the same exact code for those two functions:
Therefore, we should always favor the readable one.

<!-- new_lines: 2 -->

```rust
// Unclear way to check if this number is odd.
fn is_odd_bitwise(n: i32) -> bool {
    (n & 1) == 1
}

// The clear and readable way, using the modulo operator.
fn is_odd_modulo(n: i32) -> bool {
    n % 2 != 0
}
```
<!-- end_slide -->

Understanding Memory 
====

<!-- alignment: center -->

It's important to think about how your code uses memory at runtime. This understanding helps you prevent unnecessary memory allocations and find ways to optimize the code by avoiding them where possible.

<!-- pause -->

<!-- new_lines: 1 -->
# Show case: Memory Use When Collecting a Vector
<!-- new_lines: 1 -->

Here, we'll look at the memory allocations that happen when you call `collect::<Vec<_>>()` on an iterator. To demonstrate this, we'll write a simple memory allocator that prints a message every time the vector needs more memory.
<!-- new_lines: 1 -->

<!-- column_layout: [1, 14, 14, 1] -->

<!-- column: 1 -->
```rust
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
    hint::black_box,
};

thread_local! {
    static COUNTER: Cell<usize> = Cell::new(0);
}

struct MyAllocator;

// Implement a wrapper around System memory allocator which counts how many times
// memory has been allocated.
unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            COUNTER.set(COUNTER.get() + 1);
            System.alloc(layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;
```

<!-- column: 2 -->
```rust
fn main() {
    // Provide iterator without known length at compile time.
    let iter = (0..=16).filter(|n| *n >= black_box(0));

    // Call collect keeping track on memory alloc calls before and after
    let count_start = COUNTER.get();
    let _vec: Vec<_> = iter.collect();
    let count_end = COUNTER.get();

    println!("Before Collect. Alloc Count: {count_start}");
    println!("After Collect. Alloc Count: {count_end}");
}
```

<!-- pause -->

<!-- new_lines: 2 -->
### Output:
<!-- new_lines: 2 -->

```
Before Collect. Alloc Count: 0           
After Collect. Alloc Count: 4
```

<!-- end_slide -->

Avoid Collect
====

<!-- alignment: center -->
Here is a demonstration of some of the common pattern of misusing collect.


<!-- new_lines: 1 -->
# Is Iterator Empty
<!-- new_lines: 1 -->

```rust
// Allocate the whole iterator to know if it's empty
fn is_empty_bad<T>(iter: impl Iterator<Item = T>) -> bool {
    let vec: Vec<T> = iter.collect();
    vec.len() > 0
}
```
<!-- pause -->
<!-- new_lines: 1 -->

```rust
// No allocation + No consumption of the iterator.
fn is_empty_good<T>(iter: impl Iterator<Item = T>) -> bool {
    iter.peekable().peek().is_some()
}
```
<!-- pause -->

<!-- new_lines: 1 -->
# Check Third Item
<!-- new_lines: 1 -->

```rust
// Allocate the whole iterator to check the value of third element.
fn is_third_positive_bad(iter: impl Iterator<Item = i32>) -> bool {
    let vec: Vec<_> = iter.collect();
    vec.get(2).is_some_and(|num| num.is_positive())
}
```
<!-- pause -->
<!-- new_lines: 1 -->

```rust
// No allocation
fn is_third_positive_good(mut iter: impl Iterator<Item = i32>) -> bool {
    iter.nth(2).is_some_and(|num| num.is_positive())
}
```

<!-- end_slide -->

Avoid Collect II
====
<!-- alignment: center -->

<!-- new_lines: 1 -->
# Check if iterator contains an element.
<!-- new_lines: 1 -->

```rust
// Allocate the whole iterator to know if it contains an item
fn contain_bad<T: Eq>(iter: impl Iterator<Item = T>, item: T) -> bool {
    let vec: Vec<T> = iter.collect();
    vec.contains(&item)
}
```
<!-- pause -->
<!-- new_lines: 1 -->

```rust
// No Allocation.
fn contain_good<T: Eq>(mut iter: impl Iterator<Item = T>, item: T) -> bool {
    iter.find(|i| i == &item).is_some()
}
```
<!-- pause -->

<!-- new_lines: 1 -->
# Favor returning iterator over vector.
<!-- new_lines: 1 -->

```rust
// Unnecessary memory allocation.
fn postive_items_bad(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .filter_map(|num| num.is_positive().then_some(*num))
        .collect()
}
```
<!-- pause -->
<!-- new_lines: 1 -->

```rust
// No memory allocation. Caller can collect if needed.
fn postive_items_good(nums: &[i32]) -> impl Iterator<Item = i32> {
    nums.iter()
        .filter_map(|num| num.is_positive().then_some(*num))
}
```

<!-- end_slide -->

Reuse Memory
====
<!-- alignment: center -->
A common performance pitfall, especially in loops, is repeatedly allocating new memory for tasks. A much more efficient pattern is to reuse a single buffer. By creating a collection like a String or Vec once outside the loop, and then simply clearing it on each iteration, you can avoid the high cost of repeated memory allocations.

<!-- pause -->

<!-- column_layout: [1, 14, 14, 1] -->

<!-- column: 1 -->

<!-- new_lines: 1 -->
# Repeated Allocation:
<!-- new_lines: 1 -->
```rust
// *** External Functions ***

fn get_items() -> impl Iterator<Item = i32> {
    (0..1000).into_iter()
}

fn process_data(nums: &[i32]) {
    // ...
}

// *** Our implementation ***
fn process(limit: usize) {
    for _ in 0..limit {
        // Allocation on each loop.
        let items: Vec<_> = get_items().collect();
        process_data(&items);
    }
}
```

<!-- pause -->

<!-- column: 2 -->

<!-- new_lines: 1 -->
# Memory Reuse:
<!-- new_lines: 1 -->
```rust
// *** External Functions ***

fn get_items() -> impl Iterator<Item = i32> {
    (0..1000).into_iter()
}

fn process_data(nums: &[i32]) {
    // ...
}

// *** Our implementation ***
fn process_optimized(limit: usize) {
    let mut buffer = Vec::new();
    for _ in 0..limit {
        // Clearing the vector won't reset its capacity.
        buffer.clear();
        buffer.extend(get_items());

        process_data(&buffer);
    }
}
```

<!-- end_slide -->

Vector::Remove() Overhead
====
<!-- alignment: center -->

It's important to understand the performance overhead caused by memory rearrangement in methods on standard library collections.
Specifically, we will demonstrate the overhead of `Vec::remove()`. Removing an item from a Vector causes all subsequent elements to be shifted back one position, which can be very costly in hot loops.
Here is a illustration to the process of remove.

<!-- new_lines: 1 -->
# Initial State

```md

index: 0   1   2   3   4   5
      ┌───┬───┬───┬───┬───┬───┐
value:│ 10│ 20│ 30│ 40│ 50│ 60│
      └───┴───┴───┴───┴───┴───┘

```

<!-- new_lines: 1 -->
# Vec::remove(v, 2)
```md

      // Step 2: Shift all subsequent elements left
      //         <--- <--- <---
      ┌───┬───┬───┬───┬───┬───┐
value:│ 10│ 20│   │ 40│ 50│ 60│ // Step 1: Remove element
      └───┴───┴───┴───┴───┴───┘

```

<!-- new_lines: 1 -->
# Final State.

```md
index: 0   1   2   3   4
      ┌───┬───┬───┬───┬───┐
value:│ 10│ 20│ 40│ 50│ 60│
      └───┴───┴───┴───┴───┘
```


We will discuss next how to avoid this overhead.

<!-- end_slide -->

Vector::Swap_remove
====
<!-- alignment: center -->
Swap_Remove method will avoid shifting all the element be replacing the last element of the collection with the removed item


<!-- new_lines: 1 -->
# Vec::swap_remove(v, 2)
```md
      // The last element moves to replace the removed one.
      //        ┌──────────┐
      //        │          │
      //        ▼          ▼
      ┌───┬───┬───┬───┬───┬───┐
value:│ 10│ 20│ 30│ 40│ 50│ 60│ // `30` is replaced by `60`
      └───┴───┴───┴───┴───┴───┘

```

<!-- new_lines: 1 -->
# Final State.

```md
index: 0   1   2   3   4
      ┌───┬───┬───┬───┬───┐
value:│ 10│ 20│ 60│ 40│ 50│
      └───┴───┴───┴───┴───┘
```

<!-- end_slide -->

Vector for Stack & VecDeque for Queue
====
<!-- alignment: center -->
While a `Vec` is efficient as a **stack** (last-in, first-out), using it as a **queue** (first-in, first-out) is very slow because it must shift elements when one is removed from the front. For an efficient queue, `VecDeque` is the correct data structure.

A `VecDeque` is a "double-ended queue," designed for fast push and pop operations from both its front and back.
<!-- new_lines: 1 -->

<!-- column_layout: [1, 14, 14, 1] -->

<!-- column: 1 -->
# Vector::Remove(0)
<!-- new_lines: 1 -->

```md
Initial Vec:
      ┌───┬───┬───┬───┐
      │ 10│ 20│ 30│ 40│
      └───┴───┴───┴───┘

Action `remove(0)`:
      // Inefficient: All other elements must shift left.
      //   <--- <--- <---
      ┌───┬───┬───┬───┐
      │   │ 20│ 30│ 40│ // `10` is removed, a gap is left.
      └───┴───┴───┴───┘

Final Vec:
      ┌───┬───┬───┐
      │ 20│ 30│ 40│
      └───┴───┴───┘
```

<!--pause-->

<!-- column: 2 -->

# VecDeque::pop_front()
<!-- new_lines: 1 -->

```
Initial Deque:
    (start)
      ▼
      ┌───┬───┬───┬───┐
      │ 10│ 20│ 30│ 40│
      └───┴───┴───┴───┘

Action `pop_front()`:
      // Efficient: The internal 'start' pointer just moves.
      // No elements are shifted.
        (new start)
          ▼
      ┌───┬───┬───┬───┐
      │ 10│ 20│ 30│ 40│
      └───┴───┴───┴───┘

Final Deque:
      ┌───┬───┬───┐
      │ 20│ 30│ 40│
      └───┴───┴───┘
```

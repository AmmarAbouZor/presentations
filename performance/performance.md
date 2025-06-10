---
title: Performance Optmizations in Rust.
---

Optimize what matters:
----

<!-- alignment: center -->

Performance Optimizations requires efforts and times, therefore it's crucial to optimize the right code blocks.

<!--pause-->

<!-- new_line -->

## Example

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
----

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
----
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

Compiler Optimizations I
----
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

Compiler Optimizations II
----

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

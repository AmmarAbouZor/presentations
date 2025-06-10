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


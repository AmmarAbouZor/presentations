# Plan:

* Optimize what matters. Profiling tools [samply](https://github.com/mstange/samply)

* Always measure
  - [Criterion](https://github.com/bheisler/criterion.rs) In code.
  - [Hyperfine](https://github.com/sharkdp/hyperfine) for CLIs.

* Compiler Optimizations are really good, so try to give the compiler all informations when possible.

* Understanding memory. 
  - Demonstrate what happen on collect function. [DHAT](https://docs.rs/dhat/latest/dhat/) can be useful for memory allocations.
  - Common mistakes where collect() is used.
  - Reuse memory example
  - Vector. `remove` vs `swap_remove`
  - VecDeque for queue

* Parallelism:
  - Run same function in parallel with rayon and par_iter()
  - Find pattern where work can be shared between multiple thread. Example: Process -> Validate -> Write.

* Assertion to avoid bound checks.

* Optimize for branch prediction in processors.

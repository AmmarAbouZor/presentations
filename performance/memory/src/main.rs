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

use std::{
    alloc::{GlobalAlloc, Layout, System},
    hint::black_box,
};

static mut COUNTER: usize = 0;

struct MyAllocator;

// Implement a wrapper around System memory allocator which counts how many times
// memory has been allocated.
unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            COUNTER += 1;
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
    let count_start = unsafe { COUNTER };
    let _vec: Vec<_> = iter.collect();
    let count_end = unsafe { COUNTER };

    println!("Before Collect. Alloc Count: {count_start}");
    println!("After Collect. Alloc Count: {count_end}");
}

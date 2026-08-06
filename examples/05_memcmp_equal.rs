//! memcmp of two identical buffers; expect 0.

use rust_linux_memory_management::{allocate_memory, compare_memory, deallocate_memory};

fn main() {
    let size = 10;
    let ptr1 = allocate_memory(size);
    let ptr2 = allocate_memory(size);

    unsafe {
        for i in 0..size {
            *ptr1.add(i) = (i + 1) as u8;
            *ptr2.add(i) = (i + 1) as u8;
        }
    }

    let result = compare_memory(ptr1, ptr2, size);
    println!("memcmp result: {result}");

    deallocate_memory(ptr1);
    deallocate_memory(ptr2);
}

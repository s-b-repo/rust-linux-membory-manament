//! Allocate 10 bytes, populate, then grow to 20 with realloc and fill
//! the new tail.

use rust_linux_memory_management::{allocate_memory, deallocate_memory, reallocate_memory};

fn main() {
    let size = 10;
    let ptr = allocate_memory(size);

    unsafe {
        for i in 0..size {
            *ptr.add(i) = (i + 1) as u8;
        }
    }

    for i in 0..size {
        println!("{}", unsafe { *ptr.add(i) });
    }

    let new_size = 20;
    let new_ptr = reallocate_memory(ptr, new_size);

    unsafe {
        for i in size..new_size {
            *new_ptr.add(i) = (i + 1) as u8;
        }
    }

    for i in 0..new_size {
        println!("{}", unsafe { *new_ptr.add(i) });
    }

    deallocate_memory(new_ptr);
}

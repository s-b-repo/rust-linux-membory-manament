//! Allocate a 10-byte buffer with malloc, write 1..=10 into it, print it,
//! then free.

use rust_linux_memory_management::{allocate_memory, deallocate_memory};

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

    deallocate_memory(ptr);
}

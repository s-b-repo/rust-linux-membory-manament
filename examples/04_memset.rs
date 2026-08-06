//! memcpy then memset the destination back to zero.

use rust_linux_memory_management::{
    allocate_memory, copy_memory, deallocate_memory, set_memory,
};

fn main() {
    let size = 10;
    let ptr = allocate_memory(size);

    unsafe {
        for i in 0..size {
            *ptr.add(i) = (i + 1) as u8;
        }
    }

    let new_size = 20;
    let new_ptr = allocate_memory(new_size);
    copy_memory(ptr, new_ptr, size);

    unsafe {
        for i in size..new_size {
            *new_ptr.add(i) = (i + 1) as u8;
        }
    }

    println!("Before memset:");
    for i in 0..new_size {
        println!("  {}", unsafe { *new_ptr.add(i) });
    }

    set_memory(new_ptr, 0, new_size);

    println!("After memset(0):");
    for i in 0..new_size {
        println!("  {}", unsafe { *new_ptr.add(i) });
    }

    deallocate_memory(ptr);
    deallocate_memory(new_ptr);
}

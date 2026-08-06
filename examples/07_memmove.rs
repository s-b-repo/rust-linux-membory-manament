//! memmove — same signature as memcpy, but safe when regions overlap.
//! This demo runs it twice on separate allocations.

use rust_linux_memory_management::{allocate_memory, deallocate_memory, move_memory};

fn main() {
    let size = 10;
    let ptr = allocate_memory(size);

    unsafe {
        for i in 0..size {
            *ptr.add(i) = (i + 1) as u8;
        }
    }

    let new_ptr = allocate_memory(size);
    move_memory(ptr, new_ptr, size);

    println!("Copy A:");
    for i in 0..size {
        println!("  {}", unsafe { *new_ptr.add(i) });
    }

    let src_ptr = allocate_memory(size);
    let dest_ptr = allocate_memory(size);

    unsafe {
        for i in 0..size {
            *src_ptr.add(i) = (i + 11) as u8;
        }
    }

    move_memory(src_ptr, dest_ptr, size);

    println!("Copy B:");
    for i in 0..size {
        println!("  {}", unsafe { *dest_ptr.add(i) });
    }

    deallocate_memory(src_ptr);
    deallocate_memory(dest_ptr);
    deallocate_memory(ptr);
    deallocate_memory(new_ptr);
}

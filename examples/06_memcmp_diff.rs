//! memcmp of two buffers that differ only in their last byte.

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
        // Bump the last byte of ptr2 so ptr1 < ptr2.
        *ptr2.add(size - 1) = 11;
    }

    let result = compare_memory(ptr1, ptr2, size);
    if result < 0 {
        println!("ptr1 is less than ptr2");
    } else if result > 0 {
        println!("ptr1 is greater than ptr2");
    } else {
        println!("ptr1 is equal to ptr2");
    }

    deallocate_memory(ptr1);
    deallocate_memory(ptr2);
}

//! Thin, deliberately-unsafe wrappers around `libc` allocation and memory
//! primitives, extracted from a single-file teaching dump so each demo
//! (in `examples/`) can `use rust_linux_memory_management::*;` and stay
//! focused on one primitive at a time.
//!
//! None of this is idiomatic Rust — `Vec`, `Box`, `std::alloc::alloc`
//! and `slice::from_raw_parts_mut` are what you'd reach for in real
//! code. The point here is to show what the underlying C-level calls
//! look like when driven from Rust.

use std::ptr;

/// Allocate `size` bytes with `libc::malloc`. Panics on failure.
pub fn allocate_memory(size: usize) -> *mut u8 {
    unsafe {
        let ptr = libc::malloc(size) as *mut u8;
        if ptr.is_null() {
            panic!("Error allocating memory");
        }
        ptr
    }
}

/// Free memory previously obtained from [`allocate_memory`] /
/// [`reallocate_memory`].
///
/// # Safety
/// `ptr` must have been returned by malloc/realloc and not already freed.
pub fn deallocate_memory(ptr: *mut u8) {
    unsafe {
        libc::free(ptr as *mut libc::c_void);
    }
}

/// Grow or shrink an existing allocation with `libc::realloc`. Panics on
/// failure. The returned pointer may differ from `ptr` — do not use `ptr`
/// afterwards.
pub fn reallocate_memory(ptr: *mut u8, size: usize) -> *mut u8 {
    unsafe {
        let new_ptr = libc::realloc(ptr as *mut libc::c_void, size) as *mut u8;
        if new_ptr.is_null() {
            panic!("Error reallocating memory");
        }
        new_ptr
    }
}

/// `memcpy` — copy `size` bytes from `src` to `dest`. Regions must not
/// overlap; use [`move_memory`] if they might.
pub fn copy_memory(src: *const u8, dest: *mut u8, size: usize) {
    unsafe {
        libc::memcpy(
            dest as *mut libc::c_void,
            src as *const libc::c_void,
            size,
        );
    }
}

/// `memset` — write `value` into `size` bytes starting at `ptr`.
pub fn set_memory(ptr: *mut u8, value: u8, size: usize) {
    unsafe {
        libc::memset(ptr as *mut libc::c_void, value as libc::c_int, size);
    }
}

/// `memcmp` — lexicographic compare of `size` bytes. Returns 0 if equal,
/// negative if `ptr1 < ptr2`, positive if `ptr1 > ptr2`.
pub fn compare_memory(ptr1: *const u8, ptr2: *const u8, size: usize) -> i32 {
    unsafe {
        libc::memcmp(
            ptr1 as *const libc::c_void,
            ptr2 as *const libc::c_void,
            size,
        )
    }
}

/// `memmove` — copy `size` bytes from `src` to `dest`; safe for
/// overlapping regions.
pub fn move_memory(src: *const u8, dest: *mut u8, size: usize) {
    unsafe {
        libc::memmove(
            dest as *mut libc::c_void,
            src as *const libc::c_void,
            size,
        );
    }
}

// Suppress an unused-import warning if a user pulls just some helpers.
#[allow(dead_code)]
fn _keep_ptr_import_used() {
    let _: *mut u8 = ptr::null_mut();
}

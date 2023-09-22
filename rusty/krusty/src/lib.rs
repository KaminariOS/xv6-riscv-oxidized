#![no_std]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
pub mod console;
mod kernelib;
mod lang_items;
mod ring;
mod logger;

use kernelib::*;
pub use ring::sys_ring;

use linked_list_allocator::LockedHeap;
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[no_mangle]
pub unsafe extern "C" fn rust_function(ptr: u64) {
    // test_print(ptr);
}



#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(c_variadic)]
#![feature(alloc_error_handler)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;
pub mod user_ring;

pub mod logger;
// pub use shared::logger;

pub use syscall::*;
pub use user_ring::ringbuf;
use linked_list_allocator::LockedHeap;

const USER_HEAP_SIZE: usize = 32768;

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}


extern crate alloc;
use alloc::vec::Vec;
pub use alloc::string;

#[no_mangle]
#[link_section = ".text.entry"]
pub unsafe extern "C" fn _start() -> ! {
    // clear_bss();

    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as _, USER_HEAP_SIZE);
    }

    logger::init().unwrap();
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

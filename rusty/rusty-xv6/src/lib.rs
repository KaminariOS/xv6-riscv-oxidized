#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(c_variadic)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;
mod user_ring;

pub mod logger;
// pub use shared::logger;

pub use syscall::*;
pub use user_ring::ringbuf;

#[no_mangle]
#[link_section = ".text.entry"]
pub unsafe extern "C" fn _start() -> ! {
    // clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

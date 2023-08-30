#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(c_variadic)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

pub use syscall::*;

#[no_mangle]
#[link_section = ".text.entry"]
pub unsafe extern "C" fn _start() -> ! {
    // clear_bss();
    println!("From start");
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

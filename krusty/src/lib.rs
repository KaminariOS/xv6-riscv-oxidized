#![no_std]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
mod kernelib;
mod lang_items;
mod ring;

use kernelib::*;
pub use ring::sys_ring;

#[no_mangle]
pub unsafe extern "C" fn rust_function(ptr: u64) {
    // test_print(ptr);
}



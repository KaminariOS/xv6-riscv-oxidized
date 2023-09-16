#![no_std]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
mod kernelib;
mod lang_items;

use kernelib::*;

#[no_mangle]
pub unsafe extern "C" fn rust_function(ptr: u64) {
    // test_print(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn sys_ring() {
    println!("Test rc");
}

#[no_mangle]
pub unsafe extern "C" fn kprintf() {

}

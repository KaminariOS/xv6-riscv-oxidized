#![no_std]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
mod kernelib;
mod lang_items;
mod ring;

use kernelib::*;

use crate::ring::MAX_NAME_LEN;

#[no_mangle]
pub unsafe extern "C" fn rust_function(ptr: u64) {
    // test_print(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn sys_ring() {
    let mut name = [0; MAX_NAME_LEN];
    let res = argstr_sys(0, &mut name);
    let open = argraw(1) != 0;
    let addr = argraw(2);

    let lock = spin::Mutex::new(0);
    *lock.lock() += 1;
    println!("Test rc: {}", core::str::from_utf8(&name).unwrap());
}


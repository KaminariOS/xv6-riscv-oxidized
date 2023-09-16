#![no_std]
#![no_main]

use user_lib::*;

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    sys_ring(0);
    0
}

#![no_std]
#![no_main]

use log::*;
use user_lib::*;

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    logger::init().unwrap();
    info!("Test log");
    error!("Test log");
    debug!("Test log");
    trace!("Test log");
    warn!("Test log");
    let mut ptr = 0usize;
    ringbuf("test\0", true, &mut ptr);
    println!("User: Val: {:?}", ptr);
    0
}

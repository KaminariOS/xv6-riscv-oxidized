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
    let pid = fork();
    if let Ok((index, addr)) = ringbuf("test\0", true) {
        if pid != 0 {
            // let () =
            let mut exit_code = 0; 
            wait(&mut exit_code);
        } else {
        }
        ringbuf("test\0", false).unwrap();
    }
    // println!("User: Val: 0x{:x}", ptr);
    // println!("User: Val: 0x{:x}", ptr);
    0
}

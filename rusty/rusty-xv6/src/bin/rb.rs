#![no_std]
#![no_main]

use log::*;
use user_lib::*;
use user_lib::user_ring::*;

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    logger::init().unwrap();
    info!("Test log");
    error!("Test log");
    debug!("Test log");
    trace!("Test log");
    warn!("Test log");
    let pid = fork();
    let test_times = 50000000;
    if let Ok(index) = ringbuf("test\0", true) {
        if pid != 0 {
            // let () =
            let (len, base) = ringbuf_start_write(index);
            (0..test_times).for_each(|_| test_count(index));
            info!("Parent: len: {} base: 0x{:x}", len, base);
            let mut exit_code = 0; 
            wait(&mut exit_code);
        } else {
            (0..test_times).for_each(|_| test_count(index));
            let (len, base) = ringbuf_start_read(index);
            info!("Child: len: {} base: 0x{:x}", len, base);
        }
        assert_eq!(get_count(index), test_times * 2);
        ringbuf("test\0", false).unwrap();
    }
    // println!("User: Val: 0x{:x}", ptr);
    // println!("User: Val: 0x{:x}", ptr);
    0
}

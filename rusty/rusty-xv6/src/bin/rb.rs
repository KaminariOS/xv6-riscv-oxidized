#![no_std]
#![no_main]

use log::*;
use user_lib::{*, string::*};
use user_lib::user_ring::*;

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    info!("Test log");
    error!("Test log");
    debug!("Test log");
    trace!("Test log");
    warn!("Test log");
    // let test = String::new("test");
    let pid = fork();
    let test_times = 50000000;

    // let string = String::from("Test heap string");
    // println!("{}", string);
    
    if let Ok(index) = ringbuf("test\0", true) {
        if pid != 0 {
            // let () =
            let (len, base) = ringbuf_start_write(index);
            (0..test_times).for_each(|_| test_count(index));
            info!("Parent: len: {} base: 0x{:x}", len, base);
            let mut exit_code = 0; 
            wait(&mut exit_code);

            assert_eq!(get_count(index), test_times * 2);
        } else {
            (0..test_times).for_each(|_| test_count(index));
            let (len, base) = ringbuf_start_read(index);
            info!("Child: len: {} base: 0x{:x}", len, base);
        }

        ringbuf("test\0", false).unwrap();
    }
    // println!("User: Val: 0x{:x}", ptr);
    // println!("User: Val: 0x{:x}", ptr);
    0
}

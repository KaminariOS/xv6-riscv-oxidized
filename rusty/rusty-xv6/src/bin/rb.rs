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
    unsafe {
        
    if let Ok(index) = ringbuf("test\0", true) {
        if pid != 0 {
            // let () =
            let (len, base) = ringbuf_start_write(index);

            let slice = core::slice::from_raw_parts_mut(base as *mut u8, len);
            slice.iter_mut().enumerate().for_each(|(i, x)| *x = (i % 256) as _);
            ringbuf_finish_write(index, len);

            // (0..test_times).for_each(|_| test_count(index));
            info!("Parent: len: {} base: 0x{:x}", len, base);
            let mut exit_code = 0; 
            wait(&mut exit_code);

            // assert_eq!(get_count(index), test_times * 2);
        } else {
            loop {
                let (len, base) = ringbuf_start_read(index);
                if len != 0 {
                    let slice = core::slice::from_raw_parts(base as *const u8, len);
                    slice.iter().enumerate().for_each(|(i, x) | assert_eq!(*x, (i % 256) as _));
                    ringbuf_finish_read(index, len);
                    break;
                }
            }
            // (0..test_times).for_each(|_| test_count(index));
            // info!("Child: len: {} base: 0x{:x}", len, base);
        }

        ringbuf("test\0", false).unwrap();
    }
    }
    // println!("User: Val: 0x{:x}", ptr);
    // println!("User: Val: 0x{:x}", ptr);
    0
}

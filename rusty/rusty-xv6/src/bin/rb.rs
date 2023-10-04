#![no_std]
#![no_main]

use log::*;
use user_lib::user_ring::*;
use user_lib::{string::*, *};

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
                let mut total = 0;
                let max_byte = 10 * 1024 * 1024;

                let start_time = uptime();
                loop {
                    let (len, base) = ringbuf_start_write(index);
                    let slice = core::slice::from_raw_parts_mut(base as *mut u8, len);
                    slice
                        .iter_mut()
                        .enumerate()
                        .for_each(|(i, x)| *x = (i % 256) as _);
                    ringbuf_finish_write(index, len);
                    total += len;
                    if total >= max_byte {
                        break;
                    }
                }
                // (0..test_times).for_each(|_| test_count(index));
                // info!("Parent: len: {} base: 0x{:x}", len, base);
                let mut exit_code = 0;
                wait(&mut exit_code);

                let end_time = uptime();
                info!("{}", end_time - start_time);
                // assert_eq!(get_count(index), test_times * 2);
            } else {
                let mut total = 0;
                let max_byte = 10 * 1024 * 1024;
                loop {
                    let (len, base) = ringbuf_start_read(index);
                    let slice = core::slice::from_raw_parts(base as *const u8, len);
                    slice
                        .iter()
                        .enumerate()
                        .for_each(|(i, x)| assert_eq!(*x, (i % 256) as _));
                    ringbuf_finish_read(index, len);
                    total += len;
                    if total >= max_byte {
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

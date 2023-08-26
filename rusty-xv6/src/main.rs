#![no_std]
#![no_main]
#![feature(c_variadic)]

mod ulib;
use ulib::*;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
// #[start]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    unsafe {
        let mut pipes = [0; 2];
        if pipe(&mut pipes as *mut i32) != 0 {
            println!("Pipe error");
            exit(-1);
        }
        let id = fork();
        if id != 0 {
            wait(0 as *mut i32);
            println!("Hello world from Rust master!! %d uptime: %d", id, uptime());
        } else {
            println!("Hello world from Rust!! %d", id);
        }
        exit(0_i32)
    }
}

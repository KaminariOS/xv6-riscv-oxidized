#![no_std]
#![no_main]

// mod ulib;
// use ulib::*;
use user_lib::*;

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    unsafe {
        let mut pipes = [0; 2];
        if pipe(&mut pipes as *mut [i32; 2]) != 0 {
            println!("Pipe error");
            exit(-1);
        }
        let id = fork();
        exit(0_i32)
    }
}

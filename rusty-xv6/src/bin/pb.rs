#![no_std]
#![no_main]

use user_lib::*;

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    unsafe {
        let mut pipes = [0; 2];
        if pipe(&mut pipes) != 0 {
            println!("Pipe error");
            exit(-1);
        }
        let id = fork();
        const TOTAL_BYTES: usize = 10 * (1 << 20);
        // const TOTAL_U32: usize = TOTAL_BYTES / core::mem::size_of::<u32>();
        const BUFF: usize = 512;
        const TIMES: usize = TOTAL_BYTES / BUFF;
        let pointer = malloc(TOTAL_BYTES as _);
        for i in 0..TOTAL_BYTES {
            core::ptr::write(pointer.add(i), 1);
        }
        // let slice = slice::from_raw_parts_mut( pointer, TOTAL_BYTES);
        // slice.iter_mut().enumerate().for_each(|(i, x)| *x = i as u8);
        // slice[TOTAL_BYTES - 1] = 0;
        
        if id != 0 {
            let start_time = uptime();
            for i in 0..TIMES {
                let loc = pointer.add(i * BUFF);
                read(pipes[0] as _, core::slice::from_raw_parts_mut(loc, BUFF)); 
                let val = core::ptr::read(loc); 
                if val != 9 {
                    println!("Fuck, get {}", val as i32);
                }
            }
            wait(core::ptr::null_mut());
            let elapsed = uptime() - start_time;
            println!("id: {} uptime: {}", id, elapsed);
            close(pipes[0]);
            close(pipes[1]);
        } else {
            // dup(pipes[0]);
            for i in 0..TIMES {
                let loc = pointer.add(i * BUFF);
                core::ptr::write(loc, 9);
                write(pipes[1] as _, core::slice::from_raw_parts_mut(loc, BUFF as _));
            }
            close(pipes[1]);
            close(pipes[0]);
            // println!("Hello world from Rust!! %d", id);
        }
        exit(0_i32)
    }
}

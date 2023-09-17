#[allow(non_camel_case_types)]
type int = i32;
#[allow(non_camel_case_types)]
type uint = u32;

const SYSCALL_RING: usize = 22; 
const SYSCALL_WRITE: usize = 16; 
const SYSCALL_DUP: usize = 10;
const SYSCALL_OPEN: usize = 15;
const SYSCALL_CLOSE: usize = 21;
const SYSCALL_PIPE: usize = 4;
const SYSCALL_READ: usize = 5;
const SYSCALL_EXIT: usize = 2;
const SYSCALL_KILL: usize = 6;
const SYSCALL_FORK: usize = 1;
const SYSCALL_EXEC: usize = 7;
const SYSCALL_UPTIME: usize = 14;
const SYSCALL_WAITPID: usize = 3;
const SYSCALL_SBRK: usize = 12;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}


pub fn sys_ring(fd: usize) -> isize {
    syscall(SYSCALL_RING, [fd, 0, 0])
}


pub fn write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn uptime() -> isize {
    syscall(SYSCALL_UPTIME, [0; 3])
}

pub fn exit(exit_code: i32) -> ! {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0]);
    panic!("sys_exit never returns!");
}


pub fn fork() -> isize {
    syscall(SYSCALL_FORK, [0, 0, 0])
}

pub fn exec(path: &str, args: &[*const u8]) -> isize {
    syscall(
        SYSCALL_EXEC,
        [path.as_ptr() as usize, args.as_ptr() as usize, 0],
    )
}


pub fn dup(fd: usize) -> isize {
    syscall(SYSCALL_DUP, [fd, 0, 0])
}

pub fn read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(
        SYSCALL_READ,
        [fd, buffer.as_mut_ptr() as usize, buffer.len()],
    )
}

pub fn pipe(pipe: &mut [uint]) -> isize {
    syscall(SYSCALL_PIPE, [pipe.as_mut_ptr() as usize, 0, 0])
}


pub fn close(fd: u32) -> isize {
    syscall(SYSCALL_CLOSE, [fd as _, 0, 0])
}


pub fn wait(exit_code: *mut i32) -> isize {
    syscall(SYSCALL_WAITPID, [exit_code as usize, 0, 0])
}


#[no_mangle]
pub unsafe extern "C" fn sbrk(size: usize) -> isize { 
    syscall(SYSCALL_SBRK, [size, 0, 0])
}

#[allow(dead_code)]
extern "C" {
    // pub fn wait(_: *mut int) -> int;
    pub fn malloc(s: uint) -> *mut u8;
}



// #[no_mangle]
// pub static puts: unsafe extern "C" fn(s: *const u8, args: ...) = printf;

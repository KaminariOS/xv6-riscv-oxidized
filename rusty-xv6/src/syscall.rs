#[allow(non_camel_case_types)]
type int = i32;
#[allow(non_camel_case_types)]
type uint = u32;

const SYSCALL_RING: usize = 22; 

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

#[allow(dead_code)]
extern "C" {
    pub fn printf(s: *const u8, args: ...);
    pub fn fprintf(_: int, s: *const u8, args: ...);
    pub fn exit(status: int) -> int;
    pub fn fork() -> int;
    pub fn pipe(_: *mut [int; 2]) -> int;
    pub fn wait(_: *mut int) -> int;
    pub fn close(_: int) -> int;
    pub fn dup(_: int) -> int;
    pub fn uptime() -> int;
    pub fn malloc(s: uint) -> *mut u8;
    pub fn read(_: int, _: *mut u8, _: int);
    pub fn write(_: int, _: *const u8, _: int);
}



#[no_mangle]
pub static puts: unsafe extern "C" fn(s: *const u8, args: ...) = printf;

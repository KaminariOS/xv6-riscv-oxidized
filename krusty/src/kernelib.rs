
#[allow(non_camel_case_types)]
type int = i32;
#[allow(non_camel_case_types)]
type uint = u32;

#[allow(dead_code)]
extern "C" {
    // pub fn test_print(_ :u64) -> i64;
    pub fn uartputc(_: int); 
    pub fn kalloc() -> *mut u8;
    pub fn kfree(_: *mut u8);
    pub fn argraw(_: uint) -> usize;
    pub fn argstr(_: uint, _: *mut u8, max: uint) -> isize;
}

pub fn argstr_sys(n: usize, buf: &mut [u8]) -> isize {
    unsafe {
        argstr(n as _, buf.as_mut_ptr(), buf.len() as _)
    }
}



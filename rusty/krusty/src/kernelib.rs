use shared::*;

#[allow(non_camel_case_types)]
type pagetable_t = usize;

#[allow(dead_code)]
extern "C" {
    // pub fn test_print(_ :u64) -> i64;
    pub fn uartputc(_: int); 
    pub fn kalloc() -> *mut u8;
    pub fn kfree(_: *mut u8);
    pub fn argraw(_: uint) -> usize;
    pub fn argstr(_: uint, _: *mut u8, max: uint) -> isize;
    pub fn get_pagetable() -> pagetable_t;
    pub fn copyout(_: pagetable_t, dst: *mut u8, src: *const u8, len: usize) -> int;
    pub fn mappages(pagetable: pagetable_t, va: usize, size: usize, pa: usize, perm: int) -> int;
}

pub fn argstr_sys(n: usize, buf: &mut [u8]) -> isize {
    unsafe {
        argstr(n as _, buf.as_mut_ptr(), buf.len() as _)
    }
}



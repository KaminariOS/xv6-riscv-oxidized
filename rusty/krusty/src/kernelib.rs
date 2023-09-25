use shared::*;

#[allow(non_camel_case_types)]
type pagetable_t = usize;

#[allow(dead_code)]
extern "C" {
    // pub fn test_print(_ :u64) -> i64;
    pub fn uartputc_sync(_: int); 
    pub fn lock_console();
    pub fn unlock_console(); 
    pub fn kalloc() -> *mut u8;
    pub fn kfree(_: *mut u8);
    pub fn argraw(_: uint) -> usize;
    pub fn argstr(_: uint, _: *mut u8, max: uint) -> int;
    pub fn get_pagetable() -> pagetable_t;
    pub fn copyout(_: pagetable_t, dst: *mut u8, src: *const u8, len: usize) -> int;
    pub fn mappages(pagetable: pagetable_t, va: usize, size: usize, pa: usize, perm: uint) -> int;
    pub fn uvmunmap(pagetable: pagetable_t , va: usize, npages: usize, do_free: bool);
}


pub const PTE_V: uint = 1 << 0; // valid
pub const PTE_R: uint = 1 << 1;
pub const PTE_W: uint = 1 << 2;
pub const PTE_X: uint = 1 << 3;
pub const PTE_U: uint = 1 << 4; // user can access
pub const PTE_U_BUFF: uint = PTE_U | PTE_R | PTE_W;                                //

pub fn argstr_sys(n: usize, buf: &mut [u8]) -> int {
    unsafe {
        argstr(n as _, buf.as_mut_ptr(), buf.len() as _)
    }
}



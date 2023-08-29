#[allow(non_camel_case_types)]
type int = i32;
#[allow(non_camel_case_types)]
type uint = u32;

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

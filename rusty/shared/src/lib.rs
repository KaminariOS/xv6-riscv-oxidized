#![no_std]

#[allow(non_camel_case_types)]
pub type int = i32;
#[allow(non_camel_case_types)]
pub type uint = u32;

pub type Addr = usize;

pub const MAX_RINGBUFS: usize = 10;
pub const RINGBUF_SIZE: usize = 16;

pub const MAX_NAME_LEN: usize = 16;
pub const PAGE_SIZE: usize = 4 * 1 << 10;



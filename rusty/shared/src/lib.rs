#![no_std]

#[allow(non_camel_case_types)]
pub type int = i32;
#[allow(non_camel_case_types)]
pub type uint = u32;

pub type Addr = usize;

pub const MAX_RINGBUFS: usize = 16;
pub const RINGBUF_SIZE: usize = 16;

// One page for book and 2 guard pages; 
//| guard page
//| book page
//| guard page
//| doubly mapped buffer
pub const RINGBUF_MAPPED_SIZED: usize = RINGBUF_SIZE * 2 + 3;
pub const BOOK_OFFSET: usize = RINGBUF_SIZE * 2 + 1;

pub const MAX_NAME_LEN: usize = 16;
pub const PAGE_SIZE: usize = 4 * 1 << 10;

// Copied from Xv6
pub const MAXVA: usize = 1 << (9 + 9 + 9 + 12 - 1);
// TRAMPILINE | TRAPFRAME | guard pages
pub const UNUSED_TOP: usize = MAXVA - 10 * PAGE_SIZE;

pub const RINGBUFS_START: usize = UNUSED_TOP - MAX_RINGBUFS * RINGBUF_MAPPED_SIZED * PAGE_SIZE;

pub const fn get_ringbuf_start_va(i: usize) -> usize {
    i * RINGBUF_MAPPED_SIZED + RINGBUFS_START
}

pub const fn page_offset(i: usize) -> usize {
    i * PAGE_SIZE
}

// pub mod logger;

// #[macro_export]
// macro_rules! array {
//     (@accum (0, $($_es:expr),*) -> ($($body:tt)*))
//         => {array!(@as_expr [$($body)*])};
//     (@accum (1, $($es:expr),*) -> ($($body:tt)*))
//         => {array!(@accum (0, $($es),*) -> ($($body)* $($es,)*))};
//     (@accum (2, $($es:expr),*) -> ($($body:tt)*))
//         => {array!(@accum (0, $($es),*) -> ($($body)* $($es,)* $($es,)*))};
//     (@accum (3, $($es:expr),*) -> ($($body:tt)*))
//         => {array!(@accum (2, $($es),*) -> ($($body)* $($es,)*))};
//     (@accum (4, $($es:expr),*) -> ($($body:tt)*))
//         => {array!(@accum (2, $($es,)* $($es),*) -> ($($body)*))};
//     (@accum (5, $($es:expr),*) -> ($($body:tt)*))
//         => {array!(@accum (4, $($es),*) -> ($($body)* $($es,)*))};
//     (@accum (6, $($es:expr),*) -> ($($body:tt)*))
//         => {array!(@accum (4, $($es),*) -> ($($body)* $($es,)* $($es,)*))};
//     (@accum (7, $($es:expr),*) -> ($($body:tt)*))
//         => {array!(@accum (4, $($es),*) -> ($($body)* $($es,)* $($es,)* $($es,)*))};
//     (@accum (8, $($es:expr),*) -> ($($body:tt)*))
//         => {array!(@accum (4, $($es,)* $($es),*) -> ($($body)*))};
//     (@accum (16, $($es:expr),*) -> ($($body:tt)*))
//         => {array!(@accum (8, $($es,)* $($es),*) -> ($($body)*))};
//     (@accum (32, $($es:expr),*) -> ($($body:tt)*))
//         => {array!(@accum (16, $($es,)* $($es),*) -> ($($body)*))};
//     (@accum (64, $($es:expr),*) -> ($($body:tt)*))
//         => {array!(@accum (32, $($es,)* $($es),*) -> ($($body)*))};
//
//     (@as_expr $e:expr) => {$e};
//
//     [$e:expr; $n:tt] => { array!(@accum ($n, $e) -> ()) };
// }

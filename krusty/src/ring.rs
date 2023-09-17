use core::ptr;
use spin::Mutex;

const MAX_RINGBUFS: usize = 10;
const RINGBUF_SIZE: usize = 16;

pub const MAX_NAME_LEN: usize = 16;
const PAGE_SIZE: usize = 4 * 1 << 10;

#[repr(C)]
struct Book {
    reader : usize,
    write: usize,
}

#[repr(C)]
struct Page {
    page_inner: [u8; PAGE_SIZE]
}

#[repr(C)]
#[derive(Clone, Copy)]
struct RingBuf {
    ref_count: usize,
    name: [u8; MAX_NAME_LEN],
    buf: [*mut Page; RINGBUF_SIZE],
    book: *mut Book,
}


impl RingBuf {
    const fn default() -> Self {
        RingBuf {
            ref_count: 0,
            name: [0; 16],
            buf: [ptr::null_mut(); RINGBUF_SIZE],
            book: ptr::null_mut()
        }
    }
}

static mut RING_BUFS: Mutex<[Option<RingBuf>; MAX_RINGBUFS]> = Mutex::new([None; MAX_RINGBUFS]);




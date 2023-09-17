use crate::syscall::*;

const MAX_RINGBUFS: usize = 10;
const RINGBUF_SIZE: usize = 16;

struct BUFS ([Option<UserRingBuf>; MAX_RINGBUFS]);

static mut USER_RING_BUFS: BUFS = BUFS (
    [None; MAX_RINGBUFS]
);

#[derive(Clone, Copy)]
struct UserRingBuf {
    
}

impl BUFS {
    pub fn ringbuf_start_read(&mut self, ringbuf_desc: usize) -> (){

    }

    pub fn ringbuf_finish_read(&mut self, ringbuf_desc: usize, bytes: usize) {
    }

    pub fn ringbuf_start_write(&mut self, ringbuf_desc: usize) -> () {
    }

    pub fn ringbuf_finish_write(&mut self, ringbuf_desc: usize, bytes: usize) {
    }
}

pub fn ringbuf(name: &str, open: bool, addr: &mut *mut u8) -> isize {
    syscall(SYSCALL_RING, [name.as_bytes().as_ptr() as usize, open.into(), (addr as *mut *mut u8) as _])
}


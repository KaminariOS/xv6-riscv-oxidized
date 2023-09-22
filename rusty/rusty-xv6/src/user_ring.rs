use crate::syscall::*;
use shared::*;

struct RingbufsManager {
}

struct BUFS ([Option<UserRingBuf>; MAX_RINGBUFS]);

static mut USER_RING_BUFS: BUFS = BUFS (
    [None; MAX_RINGBUFS]
);

#[derive(Clone, Copy)]
struct UserRingBuf {
   // book
   // area
   // needles? 
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

pub fn ringbuf(name: &str, open: bool, addr: &mut usize) -> isize {
    syscall(SYSCALL_RING, [name.as_bytes().as_ptr() as usize, open.into(), addr as *mut usize as _])
}


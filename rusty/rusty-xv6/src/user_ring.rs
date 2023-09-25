use crate::syscall::*;
use shared::*;
use lazy_static::lazy_static;
use spin::Mutex;
use core::{array};

struct RingbufsManager {
}

struct BUFS ([Option<UserRingBuf>; MAX_RINGBUFS]);

lazy_static! {
    static ref USER_RING_BUFS: Mutex<BUFS> = Mutex::new(BUFS(array::from_fn(|_| None)));
}

#[derive(Clone, Copy)]
struct UserRingBuf {
    buf: usize,
    book: usize
}

#[repr(C)]
struct RingBufBook {
    nReads: usize,
    nWrites: usize
}

impl BUFS {
    pub fn ringbuf_start_read(&mut self, ringbuf_desc: usize) -> (usize, usize){
        let mut bufs = USER_RING_BUFS.lock();
        if let Some(user_ring_buf) = &mut bufs.0[ringbuf_desc] {
            // Access the fields nReads and nWrites
            let  book_ptr = user_ring_buf.book as *mut RingBufBook;
            if !book_ptr.is_null() {
                let book = unsafe { &mut *book_ptr };
                let n_reads = book.nReads;
                let n_writes = book.nWrites;
                return (book.nWrites - book.nReads, user_ring_buf.buf);
            } else {
                // Handle the case where the pointer is null
                println!("Pointer is null");
            }
        } else {
            // Handle the case where the Book field is None
            println!("Book is None for ringbuf_desc: {}", ringbuf_desc);
        }
        // let ring_book_ptr = bufs.0[ringbuf_desc].Book ;
        (0, 0)
    }

    pub fn ringbuf_finish_read(&mut self, ringbuf_desc: usize, bytes: usize) {
        let mut bufs = USER_RING_BUFS.lock();
        if let Some(user_ring_buf) = &mut bufs.0[ringbuf_desc] {
            // Access the fields nReads and nWrites
            let  book_ptr = user_ring_buf.book as *mut RingBufBook;
            if !book_ptr.is_null() {
                let book = unsafe { &mut *book_ptr };
                book.nReads += bytes;
            } 
        }
    }

    pub fn ringbuf_start_write(&mut self, ringbuf_desc: usize) -> (usize, usize) {
        let mut bufs = USER_RING_BUFS.lock();
        if let Some(user_ring_buf) = &mut bufs.0[ringbuf_desc] {
            // Access the fields nReads and nWrites
            let  book_ptr = user_ring_buf.book as *mut RingBufBook;
            if !book_ptr.is_null() {
                let book = unsafe { &mut *book_ptr };
                let n_reads = book.nReads;
                let n_writes = book.nWrites;
                return (get_ringbuf_size() - book.nWrites, user_ring_buf.buf);
            } else {
                // Handle the case where the pointer is null
                println!("Pointer is null");
            }
        } else {
            // Handle the case where the Book field is None
            println!("Book is None for ringbuf_desc: {}", ringbuf_desc);
        }
        // let ring_book_ptr = bufs.0[ringbuf_desc].Book ;
        (0, 0)
    }

    pub fn ringbuf_finish_write(&mut self, ringbuf_desc: usize, bytes: usize) {
        let mut bufs = USER_RING_BUFS.lock();
        if let Some(user_ring_buf) = &mut bufs.0[ringbuf_desc] {
            // Access the fields nReads and nWrites
            let  book_ptr = user_ring_buf.book as *mut RingBufBook;
            if !book_ptr.is_null() {
                let book = unsafe { &mut *book_ptr };
                book.nWrites += bytes;
            } 
        }
    }
}


pub fn ringbuf(name: &str, open: bool, addr: &mut usize) -> (isize, usize) {
    let status_code = syscall(SYSCALL_RING, [name.as_bytes().as_ptr() as usize, open.into(), addr as *mut usize as _]);
    let ringbuf_index = get_ringbuf_index(*addr);
    let user_ring_buf = UserRingBuf {
        buf: *addr,
        book: get_ringbuf_book_start_va(ringbuf_index)
    };
    let mut bufs = USER_RING_BUFS.lock();
    bufs.0[ringbuf_index] = Some(user_ring_buf);
    (status_code, ringbuf_index)
}


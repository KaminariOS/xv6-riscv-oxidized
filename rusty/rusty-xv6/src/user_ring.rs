use crate::syscall::*;
use shared::*;
use lazy_static::lazy_static;
use spin::Mutex;
use core::{array};
use core::sync::atomic::AtomicBool;

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
    nWrites: usize,
    locked: AtomicBool,
}

impl RingBufBook {
    fn get_diff(&self) -> usize {
        assert!(self.nWrites >= self.nReads);
        self.nWrites - self.nReads
    }

    fn lock(&mut self) {
        //cas
        while let Err(_) = self.locked.compare_exchange(false, true, core::sync::atomic::Ordering::SeqCst, core::sync::atomic::Ordering::SeqCst) {}
    }

    fn unlock(&mut self) {
        assert!(self.locked.load(core::sync::atomic::Ordering::SeqCst));
        self.locked.store(false, core::sync::atomic::Ordering::SeqCst)
    }
}


pub fn ringbuf_start_read(ringbuf_desc: usize) -> (usize, usize) {
    USER_RING_BUFS.lock().ringbuf_start_read(ringbuf_desc)
}

pub fn ringbuf_finish_read(ringbuf_desc: usize, bytes: usize) {
    USER_RING_BUFS.lock().ringbuf_finish_read(ringbuf_desc, bytes)
}

pub fn ringbuf_start_write(ringbuf_desc: usize) -> (usize, usize) {
    USER_RING_BUFS.lock().ringbuf_start_write(ringbuf_desc)
}

pub fn ringbuf_finish_write(ringbuf_desc: usize, bytes: usize) {
    USER_RING_BUFS.lock().ringbuf_finish_write(ringbuf_desc, bytes)
}

impl BUFS {
    pub fn ringbuf_start_read(&mut self, ringbuf_desc: usize) -> (usize, usize) {
        let mut bufs = USER_RING_BUFS.lock();
        if let Some(user_ring_buf) = &mut bufs.0[ringbuf_desc] {
            // Access the fields nReads and nWrites
            let  book_ptr = user_ring_buf.book as *mut RingBufBook;
            if !book_ptr.is_null() {
                let book = unsafe { &mut *book_ptr };
                let n_reads = book.nReads;
                let n_writes = book.nWrites;
                return (book.get_diff(), user_ring_buf.buf + (book.nReads % (PAGE_SIZE * RINGBUF_SIZE) ));
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
                return (get_ringbuf_buf_size() - (book.nWrites - book.nReads), user_ring_buf.buf + (book.nWrites % get_ringbuf_buf_size()));
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


pub fn ringbuf(name: &str, open: bool) -> Result<(usize, usize), isize> {
    let mut addr = 0;
    let status_code = syscall(SYSCALL_RING, [name.as_bytes().as_ptr() as usize, open.into(), (&mut addr) as *mut usize as _]);
    let ringbuf_index = get_ringbuf_index(addr);
    println!("Ringbuf index: {}", ringbuf_index);
    let mut bufs = USER_RING_BUFS.lock();
    if open {
        let user_ring_buf = UserRingBuf {
            buf: addr,
            book: get_ringbuf_book_start_va(ringbuf_index)
        };
        bufs.0[ringbuf_index] = Some(user_ring_buf);
    } else {
        bufs.0[ringbuf_index] = None;
    }
    if status_code != 0 {
        Err(status_code)
    } else {
        Ok((ringbuf_index, addr))
    }
}


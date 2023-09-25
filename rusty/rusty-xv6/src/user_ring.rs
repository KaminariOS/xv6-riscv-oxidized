use crate::syscall::*;
use shared::*;
use lazy_static::lazy_static;
use spin::Mutex;
use core::array;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

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
    n_reads: usize,
    n_writes: usize,
    locked: AtomicBool,
    _locked: AtomicBool,
    test_count: AtomicUsize,
}

impl RingBufBook {
    fn get_diff(&self) -> usize {
        assert!(self.n_writes >= self.n_reads);
        self.n_writes - self.n_reads
    }

    fn test_count(&mut self) {
        self.test_count.fetch_add(1, Ordering::SeqCst);
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


pub fn test_count(ringbuf_desc: usize) {
    USER_RING_BUFS.lock().test_count(ringbuf_desc)
}


pub fn get_count(ringbuf_desc: usize) -> usize {
    USER_RING_BUFS.lock().get_count(ringbuf_desc)
}

impl BUFS {
    pub fn test_count(&mut self, ringbuf_desc: usize)  {
        if let Some(user_ring_buf) = &mut self.0[ringbuf_desc] {
            let  book_ptr = user_ring_buf.book as *mut RingBufBook;
            if !book_ptr.is_null() {
                let mut book = unsafe { &mut *book_ptr };
                book.test_count();
            }
        }
    }

    pub fn get_count(&mut self, ringbuf_desc: usize) -> usize  {
        if let Some(user_ring_buf) = &mut self.0[ringbuf_desc] {
            let  book_ptr = user_ring_buf.book as *mut RingBufBook;
            if !book_ptr.is_null() {
                let mut book = unsafe { &mut *book_ptr };
                return book.test_count.load(Ordering::SeqCst)
            }
        }
        0
    }

    pub fn ringbuf_start_read(&mut self, ringbuf_desc: usize) -> (usize, usize) {
        if let Some(user_ring_buf) = &mut self.0[ringbuf_desc] {
            // Access the fields nReads and nWrites
            let  book_ptr = user_ring_buf.book as *mut RingBufBook;
            if !book_ptr.is_null() {
                let book = unsafe { &mut *book_ptr };
                book.lock();
                let res = (book.get_diff(), user_ring_buf.buf + (book.n_reads % (PAGE_SIZE * RINGBUF_SIZE) ));
                book.unlock();
                return res
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
        if let Some(user_ring_buf) = &mut self.0[ringbuf_desc] {
            // Access the fields nReads and nWrites
            let  book_ptr = user_ring_buf.book as *mut RingBufBook;
            if !book_ptr.is_null() {
                let book = unsafe { &mut *book_ptr };
                book.lock();
                book.n_reads += bytes;
                book.unlock();
            } 
        }
    }

    pub fn ringbuf_start_write(&mut self, ringbuf_desc: usize) -> (usize, usize) {
        if let Some(user_ring_buf) = &mut self.0[ringbuf_desc] {
            // Access the fields nReads and nWrites
            let  book_ptr = user_ring_buf.book as *mut RingBufBook;
            if !book_ptr.is_null() {
                let book = unsafe { &mut *book_ptr };
                book.lock();
                let res = (get_ringbuf_buf_size() - (book.n_writes - book.n_reads), user_ring_buf.buf + (book.n_writes % get_ringbuf_buf_size())); 
                book.unlock();
                return res;
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
        if let Some(user_ring_buf) = &mut self.0[ringbuf_desc] {
            // Access the fields nReads and nWrites
            let  book_ptr = user_ring_buf.book as *mut RingBufBook;
            if !book_ptr.is_null() {
                let book = unsafe { &mut *book_ptr };
                book.lock();
                book.n_writes += bytes;
                book.unlock();
            } 
        }
    }
}


pub fn ringbuf(name: &str, open: bool) -> Result<usize, isize> {
    let mut addr = 0;
    let status_code = syscall(SYSCALL_RING, [name.as_bytes().as_ptr() as usize, open.into(), (&mut addr) as *mut usize as _]);
    let ringbuf_index = get_ringbuf_index(addr);
    // println!("Ringbuf index: {}", ringbuf_index);
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
        Ok(ringbuf_index)
    }
}


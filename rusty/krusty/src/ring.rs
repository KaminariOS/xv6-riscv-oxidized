use core::{ptr, mem, array};
use spin::Mutex;
use crate::kernelib::*;
use shared::*;
use lazy_static::lazy_static;

#[repr(C)]
struct Book {
    reader : usize,
    write: usize,
}

#[repr(transparent)]
struct Page {
    page_inner: *mut u8
}

unsafe impl Send for Page{}

impl Page {
    fn new() -> Self {
        unsafe {
            Self {
                page_inner: kalloc()
            }
        }
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        unsafe {
            kfree(self.page_inner);
        }
    }
}

#[repr(C)]
struct RingBuf {
    ref_count: usize,
    name: [u8; MAX_NAME_LEN],
    buf: [Page; RINGBUF_SIZE],
    book: Page,
}

impl RingBuf {
    fn new(name: BufferName) -> Self 
        {
            let ringbuf = RingBuf {
                ref_count: 1,
                name,
                buf: array::from_fn(|_| Page::new()),
                book: Page::new()
            };
            ringbuf
    }
}

type BufferName =[u8; MAX_NAME_LEN];

struct RingBufInner([Option<RingBuf>; MAX_RINGBUFS]);

impl RingBufInner {
    fn close(&mut self, name: BufferName) {
        let mut found = None;
        for (i, buf) in self.0.iter_mut().enumerate() {
            if let Some(buf) = 
                buf.as_mut().filter(|b| b.name == name) {
                found = Some((i, buf));
                break;
            }
        }

        let (index, buf) = found.expect("Ring must exist"); 
        let count = buf.ref_count;
        assert!(count > 0);
        if count == 1 {
            self.0[index] = None;
        } else {
            buf.ref_count = count - 1;
        } 
    }

    fn open(&mut self, name: BufferName) -> Option<Addr> {
        let found = self.0
            .iter_mut()
            .filter_map(|i| i.as_mut())
            .find(|b| 
                    b.name == name);
        if let Some(buf) = found {
            buf.ref_count += 1;
            // map again?
            // return Some(buf.buf);
        } else {
            let slot = self.0.iter_mut().find(|slot| slot.is_none());
            if let Some(slot) = slot {
                let ringbuf = RingBuf::new(name);                
                // map
                *slot = Some(ringbuf);
                // return mapped addr
            } else {
                return None;
            }
        }
        None
    }
}

lazy_static! {
    static ref RING_BUFS: Mutex<RingBufInner> = Mutex::new(RingBufInner(array::from_fn(|_| None)));
}

unsafe fn copyout_addr(dst: *mut u8, src: *const u8) {
    let pagetable = get_pagetable();
    copyout(pagetable, dst, src, mem::size_of::<usize>());
}

#[no_mangle]
pub unsafe extern "C" fn sys_ring() -> usize {
    let mut name = [0; MAX_NAME_LEN];
    let res = argstr_sys(0, &mut name);
    if res == -1 {
        println!("Failed to get name");
        return 1;
    }
    let open = argraw(1) != 0;
    let addr = argraw(2) as *mut u8;

    let mut bufs = RING_BUFS.lock();
    if open {
        bufs.open(name);
        // alloc map and write addr
    } else {
        bufs.close(name);
    }
    let addr_new = 8usize;
    copyout_addr(addr, &addr_new as *const usize as _);
    println!("❤️ Kernel Test rc: {}", core::str::from_utf8(&name).unwrap());
    0
}

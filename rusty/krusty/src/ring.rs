use core::{ptr, mem, array};
use log::{info, warn, error};
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

    fn map(&self, start_va: usize) {
        unsafe {
        let pagetable = get_pagetable();
        self.buf.iter()
            .enumerate().
            for_each(|(i, page)| 
                {
                let mut res = mappages(
                    pagetable, 
                         start_va + page_offset(i), 
                         PAGE_SIZE, 
                         page.page_inner as _, 
                         PTE_U_BUFF);
                // info!("First mapping");
                if res != 0 {
                    error!("Mapping error first");
                }

                let res = mappages(
                    pagetable, 
                         start_va + page_offset(i) + RINGBUF_SIZE * PAGE_SIZE, 
                         PAGE_SIZE, 
                         page.page_inner as _, 
                         PTE_U_BUFF);

                // info!("Second mapping");
                if res != 0 {
                    error!("Mapping error second");
                }
                }

        );

                let res = mappages(pagetable, 
                                   start_va + BOOK_OFFSET * PAGE_SIZE, 
                                   PAGE_SIZE,
                                    self.book.page_inner as _,
                                    PTE_U_BUFF
                                   );

                // info!("Book mapping");
                if res != 0 {
                    error!("Mapping error book");
                }
        }
    }

    fn unmap(&self, start_va: usize) {
        unsafe {
        let pagetable = get_pagetable();
        uvmunmap(pagetable, start_va, RINGBUF_SIZE * 2, false);
        uvmunmap(pagetable, start_va + BOOK_OFFSET * PAGE_SIZE, 1, false);
        // info!("Unmapping");
        }
    }
}

type BufferName =[u8; MAX_NAME_LEN];

struct RingBufInner([Option<RingBuf>; MAX_RINGBUFS]);

impl RingBufInner {
    fn close(&mut self, name: BufferName) -> Addr {
        let mut found = None;
        for (i, buf) in self.0.iter_mut().enumerate() {
            if let Some(buf) = 
                buf.as_mut().filter(|b| b.name == name) {
                found = Some((i, buf));
                break;
            }
        }
        // Need to unmap
        let (index, buf) = found.expect("Ring must exist"); 
        let count = buf.ref_count;
        buf.unmap(get_ringbuf_start_va(index));
        assert!(count > 0);
        if count == 1 {
            self.0[index] = None;
        } else {
            buf.ref_count = count - 1;
        } 

        get_ringbuf_start_va(index)
    }

    fn open(&mut self, name: BufferName) -> Option<Addr> {
        let found = self.0
            .iter_mut()
            .enumerate()
            .filter_map(|(i, b)| {
                if let Some(b) = b.as_mut() {
                    Some((i, b))
                } else {None}
            })
            .find(|(_, b) | 
                    b.name == name);
        if let Some((i, buf)) = found {
            buf.ref_count += 1;
            let start_va = get_ringbuf_start_va(i);
            buf.map(start_va);
            return Some(start_va)
            // return Some(buf.buf);
        } else {
            let slot = self.0.iter_mut()
                .enumerate()
                .find(|(_, slot)| slot.is_none());
            if let Some((i, slot)) = slot {
                let ringbuf = RingBuf::new(name);                
                // map
                let start_va = get_ringbuf_start_va(i);
                ringbuf.map(start_va);
                *slot = Some(ringbuf);
                return Some(start_va)
                // return mapped addr
            } else {
                error!("Error no buf");
                return None;
            }
        }
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
        error!("Failed to get name");
        return 1;
    }
    let open = argraw(1) != 0;
    let addr = argraw(2) as *mut u8;

    // info!("❤️ BufferName: {}", core::str::from_utf8(&name).unwrap());
    let mut bufs = RING_BUFS.lock();
    if open {
        if let Some(ring_start) = bufs.open(name) {
            copyout_addr(addr, &ring_start as *const usize as _);
        } else {
            return 1;
        }
        // alloc map and write addr
    } else {
        let ring_start = bufs.close(name);
        copyout_addr(addr, &ring_start as *const usize as _);
    }
    // let addr_new = 8usize;
    // copyout_addr(addr, &addr_new as *const usize as _);
    // warn!("Kernel log: test");
    0
}

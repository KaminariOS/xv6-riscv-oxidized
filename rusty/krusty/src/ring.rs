use core::{ptr, mem};
use spin::Mutex;
use crate::kernelib::*;

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

type BufferName =[u8; MAX_NAME_LEN];
type Addr = usize;

struct RingBufInner([Option<RingBuf>; MAX_RINGBUFS]);

impl RingBufInner {
    fn close(&mut self, name: BufferName) {
        // let mut found = None;
        // for (i, buf) in self.0.iter_mut().enumerate() {
        //     if let Some(ref mut buf) = 
        //         buf.filter(|b| b.name == name) {
        //         found = Some((i, buf));
        //         break;
        //     }
        // }
        //
        // let (index, buf) = found.expect("Ring must exists"); 
        // let count = buf.ref_count;
        // assert!(count > 0);
        // if count == 1 {
        //     //TODO free pages
        //     self.0[index] = None;
        // } else {
        //     buf.ref_count = count - 1;
        // } 
    }

    fn open(&mut self, name: BufferName) -> Option<Addr> {
        let found = self.0
            .iter_mut()
            .filter_map(|i| i.as_mut())
            .find(|b| 
                    b.name == name);
        if let Some(buf) = found {
            buf.ref_count += 1;
            // return Some(buf.buf);
        } else {
            let slot = self.0.iter().position(|slot| slot.is_none());
            if let Some(slot) = slot {
                // alloc
            } else {
                // panic?
            }
            unsafe {
                let data_page = kalloc();
                let book_page = kalloc();
            }
        }
        None
    }
}

static mut RING_BUFS: Mutex<RingBufInner> = Mutex::new(RingBufInner([None; MAX_RINGBUFS]));

unsafe fn copyout_addr(dst: *mut u8, src: *const u8) {
    let pagetable = get_pagetable();
    copyout(pagetable, dst, src, mem::size_of::<usize>());
}

#[no_mangle]
pub unsafe extern "C" fn sys_ring() {
    let mut name = [0; MAX_NAME_LEN];
    let res = argstr_sys(0, &mut name);
    let open = argraw(1) != 0;
    let addr = argraw(2) as *mut u8;

    let mut bufs = RING_BUFS.lock();
    let mut found = None;
    for (i, buf) in bufs.0.iter_mut().enumerate() {
        if let Some(mut buf) = buf.filter(|b| b.name == name) {
            found = Some(i);
            break;
        }
    }
    if let Some(i) = found {
        if open {
            // mmap and write addr
        } else {
            // bufs.close(i);
        }
    } else {
        if open {

            // alloc map and write addr
        } else {
            panic!("Close a nonexist ring");
        }
    }
    let addr_new = 8usize;
    copyout_addr(addr, &addr_new as *const usize as _);
    println!("Test rc: {}", core::str::from_utf8(&name).unwrap());
}

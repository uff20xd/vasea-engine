use core::{
    alloc::{
        GlobalAlloc,
        Layout,
    },
    ptr,
    sync::atomic::*,
    cmp::max,
};

use crate::libc;

static mut PAGE_SIZE: usize = 0;

pub struct Karlloc {}

impl Karlloc {
    fn page_size() -> usize {
        unsafe {
            libc::sysconf(libc::_SC_PAGE_SIZE) as usize
        }
    }
}   

unsafe impl GlobalAlloc for Karlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        if PAGE_SIZE == 0 { unsafe { PAGE_SIZE = Self::page_size(); }}
        let aligned_layout = match layout.align_to(max(layout.align(), PAGE_SIZE)) {
            Ok(l) => l.pad_to_align(),
            Err(_) => return ptr::null_mut(),
        };
        let addr = libc::mmap(
            ptr::null_mut(),
            aligned_layout.size(),
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        );
        addr as _
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if let Ok(aligned) = layout.align_to(max(layout.align(), PAGE_SIZE)) {
            libc::munmap(ptr as _, aligned.pad_to_align().size());
        }
    }
}

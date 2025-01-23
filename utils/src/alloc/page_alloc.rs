use std::{
    alloc::{AllocError, Allocator},
    cell::UnsafeCell,
    ptr::{self, NonNull, null_mut},
};

const PAGE_SIZE: usize = 4096;

const PROT_READ: i32 = 0x1;
const PROT_WRITE: i32 = 0x2;
const MAP_PRIVATE: i32 = 0x02;

#[cfg(target_os = "macos")]
const MAP_ANONYMOUS: i32 = 0x1000;
#[cfg(target_os = "linux")]
const MAP_ANONYMOUS: i32 = 0x20;

unsafe extern "C" {
    fn mmap(addr: *mut u8, length: usize, prot: i32, flags: i32, fd: i32, offset: i64) -> *mut u8;
    fn munmap(addr: *mut u8, length: usize) -> i32;
}

pub struct PageAllocator {
    current_ptr: UnsafeCell<*mut u8>,
    base_ptr: *mut u8,
    len: usize,
}

impl PageAllocator {
    pub fn new(len: usize) -> Self {
        // this is test stuff, just exit immediately if it fails
        assert_ne!(len, 0);
        // round to 4096
        let len = (len + 4095) & !4095;

        let ptr = unsafe {
            mmap(
                null_mut(),
                len,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            )
        };

        Self {
            current_ptr: ptr.into(),
            base_ptr: ptr,
            len,
        }
    }
}

impl Drop for PageAllocator {
    fn drop(&mut self) {
        unsafe {
            munmap(self.base_ptr, self.len);
        }
    }
}

unsafe impl Allocator for PageAllocator {
    fn allocate(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<ptr::NonNull<[u8]>, std::alloc::AllocError> {
        let current = unsafe { *self.current_ptr.get() };
        let offset = current.align_offset(layout.align());
        let new_ptr = unsafe { current.add(offset) };

        if (new_ptr as usize + layout.size()) > (self.base_ptr as usize + self.len) {
            return Err(AllocError);
        }

        let slice = ptr::slice_from_raw_parts_mut(new_ptr, layout.size());
        unsafe {
            *self.current_ptr.get() = new_ptr.add(layout.size());
        }
        NonNull::new(slice).ok_or(AllocError)
    }

    // no op
    unsafe fn deallocate(&self, ptr: ptr::NonNull<u8>, layout: std::alloc::Layout) {
        if ptr.as_ptr().add(layout.size()) >= *self.current_ptr.get() {
            unsafe { *self.current_ptr.get() = ptr.as_ptr() };
        }
    }
}

use std::{
    alloc::{Allocator, Global, Layout},
    ops::{Index, IndexMut},
    ptr::NonNull,
};

pub struct BumpArray<T, A: Allocator = Global> {
    len: usize,
    capacity: usize,
    ptr: NonNull<T>,
    alloc: A,
}

impl<T, A: Allocator> BumpArray<T, A> {
    pub fn new(alloc: A, len: usize) -> Self {
        let layout = Layout::array::<T>(len).unwrap();

        let new_ptr = alloc.allocate(layout).unwrap().as_ptr() as *mut T;
        let ptr = NonNull::new(new_ptr);

        let ptr = ptr.unwrap();

        Self {
            len: 0,
            capacity: len,
            ptr,
            alloc,
        }
    }

    pub fn push(&mut self, element: T) {
        assert!(self.len < self.capacity);
        let addr = unsafe { self.ptr.add(self.len) };
        unsafe { addr.write(element) };
        self.len += 1;
    }
}

impl<T, A: Allocator> Index<usize> for BumpArray<T, A> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*self.ptr.add(index).as_ptr() }
    }
}

impl<T, A: Allocator> IndexMut<usize> for BumpArray<T, A> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *self.ptr.add(index).as_ptr() }
    }
}

impl<T: PartialEq, A: Allocator> PartialEq for BumpArray<T, A> {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }

        for i in 0..self.len {
            if self[i] != other[i] {
                return false;
            }
        }

        true
    }
}

impl<T, A: Allocator> Drop for BumpArray<T, A> {
    fn drop(&mut self) {
        let layout = Layout::array::<T>(self.capacity).unwrap();
        unsafe {
            self.alloc.deallocate(self.ptr.cast(), layout);
        };
    }
}

use std::mem::{align_of, size_of};

use crate::rust_internals::raw_vec::RawVec;

pub(crate) trait PageT {
    fn new(capacity: usize) -> Self;
    fn can_alloc(&self, num: usize) -> bool;
    fn alloc(&mut self, num: usize) -> *mut u8;
    fn can_dealloc(&self, ptr: *const u8) -> bool;
    fn dealloc(&mut self, ptr: *const u8);
}

pub(crate) struct Page<T, C> {
    buffer: RawVec<T>,
    config: C,
}

impl<T, C: Config> PageT for Page<T, C> {
    fn new(capacity: usize) -> Page<T, C> {
        let buffer = RawVec::with_capacity(capacity);
        let config = C::new::<T>(buffer.ptr() as usize, buffer.capacity());

        Page { buffer, config }
    }

    #[inline]
    fn can_alloc(&self, num: usize) -> bool {
        self.config.can_alloc(num)
    }

    #[inline]
    fn alloc(&mut self, num: usize) -> *mut u8 {
        self.config.alloc(num)
    }

    fn can_dealloc(&self, ptr: *const u8) -> bool {
        self.config.can_dealloc(ptr)
    }

    fn dealloc(&mut self, ptr: *const u8) {
        self.config.dealloc(ptr);
    }
}

impl<T: ToString, C> ToString for Page<T, C> {
    fn to_string(&self) -> String {
        let mut s = String::from("\n  buffer:");

        for idx in 0..self.buffer.capacity() * size_of::<T>() {
            let b;

            unsafe {
                b = *((self.buffer.ptr() as usize + idx) as *const u8);
            }

            s.push_str(&format!(" {:03}", b));
        }

        s
    }
}

pub(crate) trait Config {
    fn new<T>(ptr: usize, capacity: usize) -> Self;
    fn can_alloc(&self, num: usize) -> bool;
    fn alloc(&mut self, num: usize) -> *mut u8;
    fn can_dealloc(&self, ptr: *const u8) -> bool;
    fn dealloc(&mut self, ptr: *const u8);
}

/// This causes the notebook to use bump allocation. This means an allocation merely increases an
/// offset as its only write operation to reserve the memory. It also means deallocating the
/// types is a no-op. This yields very high performance at the cost of being unable to deallocate
/// memory until the whole notebook is dropped.
pub struct BumpConfig {
    ptr: usize,
    capacity: usize,
    align: usize,
    pub(crate) offset: usize,
}

impl BumpConfig {
    #[inline]
    fn remaining(&self) -> usize {
        self.capacity - self.offset
    }
}

impl Config for BumpConfig {
    fn new<T>(ptr: usize, capacity: usize) -> Self {
        BumpConfig {
            ptr,
            capacity,
            align: align_of::<T>(),
            offset: 0,
        }
    }

    #[inline]
    fn can_alloc(&self, num: usize) -> bool {
        self.remaining() >= num
    }

    #[inline]
    fn alloc(&mut self, num: usize) -> *mut u8 {
        let t = self.ptr as usize + (self.offset * self.align);

        self.offset += num;
        t as *mut u8
    }

    #[inline]
    fn can_dealloc(&self, _: *const u8) -> bool {
        // do not want to indicate failure even though deallocation is a no-op
        true
    }

    #[inline]
    fn dealloc(&mut self, _: *const u8) {
        // deallocation is a no-op for bump allocation
    }
}

use std::alloc::{Allocator, Layout};

use crate::rust_internals::raw_buffer::RawBuffer;

pub(crate) trait PageT: Sized {
    fn create(layout: Layout, alloc: &dyn Allocator) -> Option<Self>;
    fn can_alloc(&self, bytes: usize) -> bool;
    fn alloc(&mut self, bytes: usize) -> *mut u8;
    fn can_dealloc(&self, ptr: *const u8) -> bool;
    fn dealloc(&mut self, ptr: *const u8);
    fn destroy(&mut self, allocator: impl Allocator);
}

pub(crate) struct Page<C> {
    buffer: RawBuffer,
    config: C,
}

impl<C: Config> PageT for Page<C> {
    fn create(layout: Layout, allocator: &dyn Allocator) -> Option<Page<C>> {
        let buffer = RawBuffer::create(layout, allocator)?;
        let config = C::new(buffer.ptr() as usize, layout);

        Some(Page { buffer, config })
    }

    #[inline(always)]
    fn can_alloc(&self, bytes: usize) -> bool {
        self.config.can_alloc(bytes)
    }

    #[inline(always)]
    fn alloc(&mut self, bytes: usize) -> *mut u8 {
        self.config.alloc(bytes)
    }

    #[inline(always)]
    fn can_dealloc(&self, ptr: *const u8) -> bool {
        self.config.can_dealloc(ptr)
    }

    #[inline(always)]
    fn dealloc(&mut self, ptr: *const u8) {
        self.config.dealloc(ptr);
    }

    fn destroy(&mut self, allocator: impl Allocator) {
        self.buffer.destroy(allocator);
    }
}

impl<C> ToString for Page<C> {
    fn to_string(&self) -> String {
        let mut s = String::from("\n  buffer:");

        for idx in 0..self.buffer.size() {
            let b;

            unsafe {
                b = *((self.buffer.ptr() as usize + idx) as *const u8);
            }

            s.push_str(&format!(" {:03}", b));
        }

        s
    }
}

pub trait Config: Send + Sync {
    fn new(ptr: usize, layout: Layout) -> Self;
    fn can_alloc(&self, bytes: usize) -> bool;
    fn alloc(&mut self, bytes: usize) -> *mut u8;
    fn can_dealloc(&self, ptr: *const u8) -> bool;
    fn dealloc(&mut self, ptr: *const u8);
}

/// This causes the notebook to use bump allocation. This means an allocation merely increases an
/// offset as its only write operation to reserve the memory. It also means deallocating the
/// types is a no-op. This yields very high performance at the cost of being unable to deallocate
/// memory until the whole notebook is dropped.
pub struct BumpConfig {
    ptr: usize,
    layout: Layout,
    pub(crate) offset: usize,
}

impl BumpConfig {
    #[inline(always)]
    fn remaining(&self) -> usize {
        self.layout.size() - self.offset
    }
}

impl Config for BumpConfig {
    fn new(ptr: usize, layout: Layout) -> Self {
        BumpConfig {
            ptr,
            layout,
            offset: 0,
        }
    }

    #[inline(always)]
    fn can_alloc(&self, bytes: usize) -> bool {
        self.remaining() >= bytes
    }

    #[inline(always)]
    fn alloc(&mut self, bytes: usize) -> *mut u8 {
        let t = self.ptr as usize + self.offset;

        self.offset += bytes;
        t as *mut u8
    }

    #[inline(always)]
    fn can_dealloc(&self, _: *const u8) -> bool {
        // do not want to indicate failure even though deallocation is a no-op
        true
    }

    #[inline(always)]
    fn dealloc(&mut self, _: *const u8) {
        // deallocation is a no-op for bump allocation
    }
}

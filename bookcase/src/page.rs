use core::alloc::Layout;
use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::allocator::BookcaseAllocator;

pub(crate) struct Page<C, T=u8> {
    ptr: NonNull<T>,
    layout: Layout,
    utensil: C,
    _owns_ptr: PhantomData<T>,
}

impl<C: Utensil> Page<C> {
    pub(crate) fn create(layout: Layout, allocator: &dyn BookcaseAllocator) -> Option<Page<C>> {
        if usize::BITS < 64 && layout.size() > isize::MAX as usize {
            return None;
        }

        let ptr = allocator.allocate(layout).ok()?.cast().as_ptr();
        let config = C::new(ptr as usize, layout);

        Some(Page {
            ptr: unsafe { NonNull::new_unchecked(ptr) },
            layout,
            utensil: config,
            _owns_ptr: PhantomData,
        })
    }

    #[cfg(test)]
    pub(crate) fn clone_buffer(&self) -> Vec<u8> {
        let mut v = Vec::new();

        for idx in 0..self.layout.size() {
            unsafe { v.push(*((self.ptr.as_ptr() as usize + idx) as *const u8)); }
        }

        v
    }

    #[inline(always)]
    pub(crate) fn can_alloc(&self, bytes: usize) -> bool {
        self.utensil.can_alloc(bytes)
    }

    #[inline(always)]
    pub(crate) fn alloc(&mut self, bytes: usize) -> *mut u8 {
        self.utensil.alloc(bytes)
    }

    #[inline(always)]
    pub(crate) fn can_dealloc(&self, ptr: *const u8) -> bool {
        self.utensil.can_dealloc(ptr)
    }

    #[inline(always)]
    pub(crate) fn dealloc(&mut self, ptr: *const u8) {
        self.utensil.dealloc(ptr);
    }

    pub(crate) fn destroy(&mut self, allocator: &dyn BookcaseAllocator) {
        unsafe {
            allocator.deallocate(self.ptr.into(), self.layout);
        }
    }
}

impl<C> ToString for Page<C> {
    fn to_string(&self) -> String {
        let mut s = String::from("\n  buffer:");

        for idx in 0..self.layout.size() {
            let b;

            unsafe {
                b = *((self.ptr.as_ptr() as usize + idx) as *const u8);
            }

            s.push_str(&format!(" {:02x}", b));
        }

        s
    }
}

pub trait Utensil: Send + Sync {
    fn new(addr: usize, layout: Layout) -> Self;
    fn can_alloc(&self, bytes: usize) -> bool;
    fn alloc(&mut self, bytes: usize) -> *mut u8;
    fn can_dealloc(&self, ptr: *const u8) -> bool;
    fn dealloc(&mut self, ptr: *const u8);
}

/// You cannot erase ink.
/// This causes the notebook to use bump allocation. This means an allocation merely increases an
/// offset as its only write operation to reserve the memory. It also means deallocating is a no-op.
/// This yields very high performance at the cost of being unable to deallocate memory until the
/// whole notebook is dropped.
pub struct Pen {
    addr: usize,
    layout: Layout,
    pub(crate) offset: usize,
}

impl Pen {
    #[inline(always)]
    fn remaining(&self) -> usize {
        self.layout.size() - self.offset
    }
}

impl Utensil for Pen {
    fn new(addr: usize, layout: Layout) -> Self {
        Pen {
            addr,
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
        let t = self.addr + self.offset;

        self.offset += bytes;
        t as *mut u8
    }

    #[inline(always)]
    fn can_dealloc(&self, _: *const u8) -> bool {
        true
    }

    #[inline(always)]
    fn dealloc(&mut self, _: *const u8) {
    }
}

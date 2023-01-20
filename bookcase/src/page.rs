use core::alloc::Layout;
use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::allocator::BookcaseAllocator;
use crate::seal::Sealed;

pub(crate) struct Page<U, T=u8> {
    ptr: NonNull<T>,
    layout: Layout,
    utensil: U,
    _owns_ptr: PhantomData<T>,
}

impl<U: Utensil> Page<U> {
    pub(crate) fn create(layout: Layout, allocator: &dyn BookcaseAllocator) -> Option<Page<U>> {
        if usize::BITS < 64 && layout.size() > isize::MAX as usize {
            return None;
        }

        let ptr = allocator.allocate(layout).ok()?.cast().as_ptr();
        let utensil = U::new(ptr as usize, layout);

        Some(Page {
            ptr: unsafe { NonNull::new_unchecked(ptr) },
            layout,
            utensil,
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

unsafe impl<U: Utensil> Send for Page<U, u8> {}

impl<U> ToString for Page<U> {
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

pub trait Utensil: Send + Sync + Sealed {
    fn new(addr: usize, layout: Layout) -> Self;
    fn can_alloc(&self, bytes: usize) -> bool;
    fn alloc(&mut self, bytes: usize) -> *mut u8;
    fn can_dealloc(&self, ptr: *const u8) -> bool;
    fn dealloc(&mut self, ptr: *const u8);
}

/// You cannot erase ink.
///
/// This causes the notebook to use bump allocation. This means an allocation merely increases an
/// offset as its only write operation to reserve the memory (unless the chapter is full and needs
/// to allocate a new page). It also means deallocating is a no-op. This yields very high
/// performance at the cost of being unable to deallocate memory until the whole notebook is
/// dropped.
pub struct Pen {
    addr: usize,
    layout: Layout,
    pub(crate) offset: usize,
}

impl Sealed for Pen {}

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

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::*;

    macro_rules! new_page {
        ($layout:ty) => {
            Page::<crate::Pen>::create(
                std::alloc::Layout::new::<$layout>(),
                &crate::StdAllocator,
            ).expect(line_str!())
        };
        ($layout:ty, $utensil:ty) => {
            Page::<crate::$utensil>::create(
                std::alloc::Layout::new::<$layout>(),
                &crate::StdAllocator,
            ).expect(line_str!())
        };
    }

    #[test]
    fn can_allocate_full_size() {
        assert!(new_page!(usize).can_alloc(size_of::<usize>()));
    }

    #[test]
    fn cannot_allocate_greater_than_full_size() {
        assert!(!new_page!(usize).can_alloc(size_of::<usize>() + 1));
    }

    #[test]
    fn can_allocate_multiple() {
        let mut page = new_page!([usize; 4]);

        assert!(page.can_alloc(size_of::<usize>()));
        page.alloc(size_of::<usize>());
        assert!(page.can_alloc(size_of::<usize>()));
        page.alloc(size_of::<usize>());
        assert!(page.can_alloc(size_of::<usize>()));
        page.alloc(size_of::<usize>());
        assert!(page.can_alloc(size_of::<usize>()));
        page.alloc(size_of::<usize>());
        assert!(!page.can_alloc(size_of::<usize>()));
    }

    #[test]
    fn can_allocate_partial() {
        let mut page = new_page!(u128);

        assert!(page.can_alloc(size_of::<u64>()));
        page.alloc(size_of::<u64>());
        assert!(page.can_alloc(size_of::<u32>()));
        page.alloc(size_of::<u32>());
        assert!(page.can_alloc(size_of::<u16>()));
        page.alloc(size_of::<u16>());
        assert!(page.can_alloc(size_of::<u8>()));
        page.alloc(size_of::<u8>());
        assert!(page.can_alloc(size_of::<u8>()));
        page.alloc(size_of::<u8>());
        assert!(!page.can_alloc(size_of::<u8>()));
    }

    #[test]
    fn pen_can_always_deallocate() {
        let mut page = new_page!(u128);

        assert!(page.can_alloc(size_of::<u64>()));
        page.alloc(size_of::<u64>());
        assert!(page.can_alloc(size_of::<u32>()));
        page.alloc(size_of::<u32>());
        assert!(page.can_alloc(size_of::<u16>()));
        page.alloc(size_of::<u16>());
        assert!(page.can_alloc(size_of::<u8>()));
        page.alloc(size_of::<u8>());
        assert!(page.can_alloc(size_of::<u8>()));
        page.alloc(size_of::<u8>());
        assert!(!page.can_alloc(size_of::<u8>()));
    }
}

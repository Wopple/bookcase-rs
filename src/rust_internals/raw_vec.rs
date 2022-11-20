use std::alloc::{Allocator, Global, Layout};
use std::ops::Drop;
use std::ptr::{NonNull, Unique};

pub(crate) struct RawVec<T, A: Allocator = Global> {
    ptr: Unique<T>,
    cap: usize,
    alloc: A,
}

impl<T> RawVec<T, Global> {
    #[inline]
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_in(capacity, Global)
    }
}

impl<T, A: Allocator> RawVec<T, A> {
    #[inline]
    pub(crate) fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        Self::allocate_in(capacity, alloc)
    }

    fn allocate_in(capacity: usize, alloc: A) -> Self {
        let layout = match Layout::array::<T>(capacity) {
            Ok(layout) => layout,
            Err(_) => capacity_overflow(),
        };

        if usize::BITS < 64 && layout.size() > isize::MAX as usize {
            capacity_overflow();
        }

        let ptr = match alloc.allocate(layout) {
            Ok(ptr) => ptr,
            Err(_) => capacity_overflow(),
        };

        Self {
            ptr: unsafe { Unique::new_unchecked(ptr.cast().as_ptr()) },
            cap: capacity,
            alloc,
        }
    }

    #[inline]
    pub(crate) fn ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    #[inline(always)]
    pub(crate) fn capacity(&self) -> usize {
        self.cap
    }

    fn current_memory(&self) -> Option<(NonNull<u8>, Layout)> {
        unsafe {
            let layout = Layout::array::<T>(self.cap).unwrap_unchecked();
            Some((self.ptr.cast().into(), layout))
        }
    }
}

unsafe impl<#[may_dangle] T, A: Allocator> Drop for RawVec<T, A> {
    /// Frees the memory owned by the `RawVec` *without* trying to drop its contents.
    fn drop(&mut self) {
        if let Some((ptr, layout)) = self.current_memory() {
            unsafe { self.alloc.deallocate(ptr, layout) }
        }
    }
}

fn capacity_overflow() -> ! {
    panic!("capacity overflow");
}

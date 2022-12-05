use std::alloc::{alloc, alloc_zeroed, dealloc, Layout, realloc};
use std::ptr::{copy_nonoverlapping as copy_bytes, NonNull};

use crate::allocator::BookcaseAllocator;
use crate::error::Error;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
struct Slice<T> {
    ptr: *mut T,
    len: usize,
}

impl<T> Slice<T> {
    fn non_null(ptr: *mut T, len: usize) -> NonNull<[T]> {
        unsafe { NonNull::new_unchecked(std::mem::transmute(Slice { ptr, len })) }
    }
}

pub(crate) struct StdAllocator;

impl BookcaseAllocator for StdAllocator {
    #[inline(always)]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, Error> {
        if layout.size() == 0 {
            unsafe {
                return Ok(Slice::non_null(std::mem::transmute(layout.align()), 0));
            }
        }

        let ptr = unsafe { alloc(layout) };

        if ptr.is_null() {
            Err(Error::AllocError)
        } else {
            Ok(Slice::non_null(ptr, layout.size()))
        }
    }

    #[inline(always)]
    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, Error> {
        let ptr = unsafe { alloc_zeroed(layout) };

        if ptr.is_null() {
            Err(Error::AllocError)
        } else {
            Ok(Slice::non_null(ptr, layout.size()))
        }
    }

    #[inline(always)]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe { dealloc(ptr.as_ptr(), layout); }
    }

    #[inline]
    unsafe fn grow(
        &self,
        old_ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, Error> {
        let old_size = old_layout.size();
        let new_size = new_layout.size();

        if old_size == 0 {
            self.allocate(new_layout)
        } else if old_size == new_size {
            Ok(Slice::non_null(old_ptr.as_ptr(), old_size))
        } else if old_size > new_size {
            Err(Error::GrowError(old_size, new_size))
        } else if old_layout.align() == new_layout.align() {
            let new_ptr = realloc(old_ptr.as_ptr(), old_layout, new_size);

            if new_ptr.is_null() {
                Err(Error::AllocError)
            } else {
                Ok(Slice::non_null(new_ptr, new_size))
            }
        } else {
            let new_ptr = self.allocate(new_layout)?;

            copy_bytes(old_ptr.as_ptr(), new_ptr.as_ptr().cast(), old_size);
            self.deallocate(old_ptr, old_layout);
            Ok(new_ptr)
        }
    }

    #[inline(always)]
    unsafe fn grow_zeroed(
        &self,
        old_ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, Error> {
        let new_ptr = self.grow(old_ptr, old_layout, new_layout)?;

        new_ptr
            .as_ptr()
            .cast::<*mut u8>()
            .add(old_layout.size())
            .write_bytes(0, new_layout.size() - old_layout.size());

        Ok(new_ptr)
    }

    #[inline]
    unsafe fn shrink(
        &self,
        old_ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, Error> {
        let old_size = old_layout.size();
        let new_size = new_layout.size();

        if old_size < new_size {
            Err(Error::ShrinkError(old_size, new_size))
        } else if old_size == new_size {
            Ok(Slice::non_null(old_ptr.as_ptr(), old_size))
        } else {
            let new_ptr = self.allocate(new_layout)?;

            copy_bytes(old_ptr.as_ptr(), new_ptr.as_ptr().cast(), new_size);
            self.deallocate(old_ptr, old_layout);
            Ok(new_ptr)
        }
    }

    #[inline(always)]
    fn by_ref(&self) -> &Self where Self: Sized {
        self
    }
}

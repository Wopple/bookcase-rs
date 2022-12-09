use std::alloc::{Global, Allocator, Layout};
use std::ptr::NonNull;
use crate::allocator::BookcaseAllocator;
use crate::error::Error;

impl<T: Allocator> BookcaseAllocator for T {
    #[inline(always)]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, Error> {
        Allocator::allocate(self, layout).map_err(Error::from)
    }

    #[inline(always)]
    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, Error> {
        Allocator::allocate_zeroed(&self, layout).map_err(Error::from)
    }

    #[inline(always)]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        Allocator::deallocate(&self, ptr, layout)
    }

    #[inline(always)]
    unsafe fn grow(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, Error> {
        Allocator::grow(&self, ptr, old_layout, new_layout).map_err(Error::from)
    }

    #[inline(always)]
    unsafe fn grow_zeroed(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, Error> {
        Allocator::grow_zeroed(&self, ptr, old_layout, new_layout).map_err(Error::from)
    }

    #[inline(always)]
    unsafe fn shrink(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, Error> {
        Allocator::shrink(&self, ptr, old_layout, new_layout).map_err(Error::from)
    }

    #[inline(always)]
    fn by_ref(&self) -> &Self where Self: Sized {
        Allocator::by_ref(&self)
    }
}

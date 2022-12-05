use std::alloc::{Global, Allocator, Layout};
use std::ptr::NonNull;
use crate::allocator::BookcaseAllocator;
use crate::error::Error;

impl BookcaseAllocator for Global {
    #[inline(always)]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, Error> {
        Allocator::allocate(&Global, layout).map_err(Error::from)
    }

    #[inline(always)]
    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, Error> {
        Allocator::allocate_zeroed(&Global, layout).map_err(Error::from)
    }

    #[inline(always)]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        Allocator::deallocate(&Global, ptr, layout)
    }

    #[inline(always)]
    unsafe fn grow(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, Error> {
        Allocator::grow(&Global, ptr, old_layout, new_layout).map_err(Error::from)
    }

    #[inline(always)]
    unsafe fn grow_zeroed(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, Error> {
        Allocator::grow_zeroed(&Global, ptr, old_layout, new_layout).map_err(Error::from)
    }

    #[inline(always)]
    unsafe fn shrink(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, Error> {
        Allocator::shrink(&Global, ptr, old_layout, new_layout).map_err(Error::from)
    }

    #[inline(always)]
    fn by_ref(&self) -> &Self where Self: Sized {
        Allocator::by_ref(&Global, )
    }
}

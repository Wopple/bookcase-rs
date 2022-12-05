use std::alloc::Layout;
use std::ptr::NonNull;

use crate::error::Error;

#[cfg(feature = "allocator_api")]
pub(crate) mod nightly_allocator;

#[cfg(not(feature = "allocator_api"))]
pub(crate) mod stable_allocator;

pub trait BookcaseAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, Error>;
    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, Error>;
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);

    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, Error>;

    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, Error>;

    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, Error>;

    fn by_ref(&self) -> &Self where Self: Sized;
}

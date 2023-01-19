#[cfg(feature = "allocator_api")]
pub use std::alloc::Global as StdAllocator;
use std::alloc::Layout;
use std::ptr::NonNull;

#[cfg(not(feature = "allocator_api"))]
pub use stable_allocator::StdAllocator;

use crate::error::Error;

#[cfg(feature = "allocator_api")]
pub(crate) mod nightly_allocator;

#[cfg(not(feature = "allocator_api"))]
pub(crate) mod stable_allocator;

/// This trait mirrors the Allocator trait from the allocator_api. The allocator_api is only
/// available on nightly. This is an abstraction over the presence of the allocator_api to allow the
/// library to work on stable rust.
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

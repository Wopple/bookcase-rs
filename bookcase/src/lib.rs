#![cfg_attr(feature = "allocator_api", feature(allocator_api))]

pub use allocator::StdAllocator;
pub use handle::Handle;
pub use notebook::*;
pub use page::*;
pub use strategy::*;

#[cfg(not(test))]
::bookcase_alloc_macros::assert_release_channel!();

pub(crate) mod allocator;
pub(crate) mod chapter;
pub(crate) mod error;
pub(crate) mod handle;
pub(crate) mod notebook;
pub(crate) mod page;
pub(crate) mod seal;
pub(crate) mod strategy;
#[cfg(test)]
pub(crate) mod test;

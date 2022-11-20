#![feature(allocator_api)]
#![feature(dropck_eyepatch)]
#![feature(ptr_internals)]

pub use notebook::*;
pub use strategy::*;

pub(crate) mod chapter;
pub(crate) mod handle;
pub(crate) mod notebook;
pub(crate) mod page;
pub(crate) mod rust_internals;
pub(crate) mod strategy;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

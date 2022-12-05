#[derive(Copy, Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "allocator_api")]
    #[error("{0}")]
    AllocError(std::alloc::AllocError),

    #[cfg(not(feature = "allocator_api"))]
    #[error("unable to allocate")]
    AllocError,

    #[error("old size {0} is larger than new size {1}")]
    GrowError(usize, usize),

    #[error("old size {0} is smaller than new size {1}")]
    ShrinkError(usize, usize),
}

#[cfg(feature = "allocator_api")]
impl From<std::alloc::AllocError> for Error {
    fn from(error: std::alloc::AllocError) -> Self {
        Error::AllocError(error)
    }
}

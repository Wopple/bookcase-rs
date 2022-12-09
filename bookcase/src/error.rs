use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    #[cfg(feature = "allocator_api")]
    AllocError(std::alloc::AllocError),

    #[cfg(not(feature = "allocator_api"))]
    AllocError,

    GrowError(usize, usize),

    ShrinkError(usize, usize),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match self {
            #[cfg(feature = "allocator_api")]
            Error::AllocError(e) => e.fmt(fmt),

            #[cfg(not(feature = "allocator_api"))]
            Error::AllocError => fmt.write_str("memory allocation failed"),

            Error::GrowError(old, new) => fmt.write_str(&format!(
                "old size {} is larger than new size {}", old, new
            )),
            Error::ShrinkError(old, new) => fmt.write_str(&format!(
                "old size {} is smaller than new size {}", old, new
            )),
        }
    }
}

#[cfg(feature = "allocator_api")]
impl From<std::alloc::AllocError> for Error {
    fn from(error: std::alloc::AllocError) -> Self {
        Error::AllocError(error)
    }
}

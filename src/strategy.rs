use std::mem::{align_of, size_of};

/// This controls how much memory each page allocates.
#[derive(Clone, Copy, Debug)]
pub enum SizeStrategy {
    /// n * align_of::<T> bytes
    AlignmentsPerPage(usize),

    /// n * size_of::<T> bytes
    ItemsPerPage(usize),

    /// n * size_of::<usize> bytes
    WordsPerPage(usize),
}

impl SizeStrategy {
    #[inline]
    pub(crate) fn alignments<T>(&self) -> usize {
        match *self {
            SizeStrategy::AlignmentsPerPage(n) => n,
            SizeStrategy::ItemsPerPage(n) => n * size_of::<T>() / align_of::<T>(),
            SizeStrategy::WordsPerPage(n) => n * size_of::<usize>() / align_of::<T>(),
        }
    }
}

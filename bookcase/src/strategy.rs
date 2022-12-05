use core::mem::size_of;

/// This controls the base size of memory allocated for each page.
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
    #[inline(always)]
    pub(crate) fn base_bytes(&self, t_size: usize, t_align: usize) -> usize {
        match *self {
            SizeStrategy::AlignmentsPerPage(n) => n * t_align,
            SizeStrategy::ItemsPerPage(n) => n * t_size,
            SizeStrategy::WordsPerPage(n) => n * size_of::<usize>(),
        }
    }
}

/// This controls the growth rate of each page.
#[derive(Clone, Copy, Debug)]
pub enum GrowthStrategy {
    /// size_strategy bytes
    Constant,

    /// size_strategy * n * page_num bytes
    Linear(usize),

    /// size_strategy * (2 ^ page_num) bytes
    Exponential,
}

impl GrowthStrategy {
    #[inline(always)]
    pub(crate) fn page_bytes(&self, base_bytes: usize, page_idx: usize) -> usize {
        match *self {
            GrowthStrategy::Constant => base_bytes,
            GrowthStrategy::Linear(n) => base_bytes * n * (page_idx + 1),
            GrowthStrategy::Exponential => base_bytes << page_idx,
        }
    }
}

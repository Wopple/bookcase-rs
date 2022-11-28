use std::alloc::{Allocator, Layout};
use std::ptr::Unique;

/// Fixed size buffer borrowing code from rust lang's RawVec. This is not called a "vec" because
/// it does not need to grow.
pub(crate) struct RawBuffer {
    ptr: Unique<u8>,
    layout: Layout,
}

impl<'a> RawBuffer {
    pub(crate) fn create(layout: Layout, allocator: &dyn Allocator) -> Option<Self> {
        if usize::BITS < 64 && layout.size() > isize::MAX as usize {
            return None;
        }

        let non_null = allocator.allocate(layout).ok()?;

        Some(Self {
            ptr: unsafe { Unique::new_unchecked(non_null.cast().as_ptr()) },
            layout,
        })
    }

    #[inline(always)]
    pub(crate) fn ptr(&self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    #[inline(always)]
    pub(crate) fn size(&self) -> usize {
        self.layout.size()
    }

    #[inline(always)]
    pub(crate) fn align(&self) -> usize {
        self.layout.align()
    }

    pub(crate) fn destroy(&self, allocator: impl Allocator) {
        unsafe {
            allocator.deallocate(self.ptr.into(), self.layout);
        }
    }
}

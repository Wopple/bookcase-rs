use std::cell::RefCell;
use std::marker::PhantomData;
use std::mem::{align_of, size_of};
use std::sync::RwLock;

use crate::{GrowthStrategy, SizeStrategy};
use crate::allocator::BookcaseAllocator;
use crate::chapter::Chapter;
use crate::handle::Handle;
use crate::page::Utensil;
use crate::seal::Sealed;

pub trait Notebook: Sealed {
    fn alloc<T: Copy>(&self) -> Option<&mut T>;

    /// Zeroes all bytes allocated including padding.
    #[inline(always)]
    fn alloc_zero<T: Copy>(&self) -> Option<&mut T> {
        let t_ref = self.alloc()?;

        unsafe {
            (t_ref as *mut T).write_bytes(0, 1);
        }

        Some(t_ref)
    }

    /// Initializes the memory with the given value.
    #[inline(always)]
    fn alloc_init<T: Copy>(&self, t: T) -> Option<&mut T> {
        let t_ref = self.alloc()?;

        *t_ref = t;
        Some(t_ref)
    }

    /// Moves a handle to the caller which will call drop on the value when the handle is dropped.
    fn new<T>(&self, t: T) -> Option<Handle<T>> where Self: Sized;

    fn dealloc<T>(&self, t: &T) -> bool;
}

/// *_t suffix is used so as not to clash with Notebook's interface.
pub trait TypedNotebook<T>: Sealed {
    fn alloc_t(&self) -> Option<&mut T> where T: Copy;

    /// Zeroes all bytes allocated including padding.
    #[inline(always)]
    fn alloc_zero_t(&self) -> Option<&mut T> where T: Copy {
        let t_ref = self.alloc_t()?;

        unsafe {
            (t_ref as *mut T).write_bytes(0, 1);
        }

        Some(t_ref)
    }

    /// Initializes the memory with the given value.
    #[inline(always)]
    fn alloc_init_t(&self, t: T) -> Option<&mut T> where T: Copy {
        let t_ref = self.alloc_t()?;

        *t_ref = t;
        Some(t_ref)
    }

    /// Moves a handle to the caller which will call drop on the value when the handle is dropped.
    fn new_t(&self, t: T) -> Option<Handle<T>> where Self: Sized;

    fn dealloc_t(&self, t: &T) -> bool;
}

/// Allows Notebooks to be used as TypedNotebooks.
impl<N: Notebook, T> TypedNotebook<T> for N {
    #[inline(always)]
    fn alloc_t(&self) -> Option<&mut T> where T: Copy {
        self.alloc::<T>()
    }

    #[inline(always)]
    fn alloc_zero_t(&self) -> Option<&mut T> where T: Copy {
        self.alloc_zero::<T>()
    }

    #[inline(always)]
    fn alloc_init_t(&self, t: T) -> Option<&mut T> where T: Copy {
        self.alloc_init::<T>(t)
    }

    #[inline(always)]
    fn new_t(&self, t: T) -> Option<Handle<T>> {
        self.new::<T>(t)
    }

    #[inline(always)]
    fn dealloc_t(&self, t: &T) -> bool {
        self.dealloc::<T>(t)
    }
}

const NUM_ALIGNS: usize = 5;

impl Sealed for () {}
impl Sealed for RwLock<()> {}

/// Can allocate any type. All types will be allocated to their proper alignment. This is
/// especially useful for processing heterogeneous granular data like parsing a JSON string
/// by minimizing the frequency of calling into the operating system for allocation.
pub struct MultiNotebook<A: BookcaseAllocator, U: Utensil, L = ()> {
    allocator: A,
    size: SizeStrategy,
    growth: GrowthStrategy,
    chapters: RefCell<[Chapter<U>; NUM_ALIGNS]>,
    lock: L,
}

impl<A: BookcaseAllocator, U: Utensil, L: Sealed> Sealed for MultiNotebook<A, U, L> {}

// These implementations are to reduce duplication.
impl<A: BookcaseAllocator, U: Utensil, L> MultiNotebook<A, U, L> {
    #[cfg(test)]
    pub(crate) fn clone_chapters_impl(&self) -> [Vec<Vec<u8>>; NUM_ALIGNS] {
        [
            self.chapters.borrow().get(0).unwrap().pages().iter().map(|p: &crate::page::Page<U>| {
                p.clone_buffer()
            }).collect(),
            self.chapters.borrow().get(1).unwrap().pages().iter().map(|p: &crate::page::Page<U>| {
                p.clone_buffer()
            }).collect(),
            self.chapters.borrow().get(2).unwrap().pages().iter().map(|p: &crate::page::Page<U>| {
                p.clone_buffer()
            }).collect(),
            self.chapters.borrow().get(3).unwrap().pages().iter().map(|p: &crate::page::Page<U>| {
                p.clone_buffer()
            }).collect(),
            self.chapters.borrow().get(4).unwrap().pages().iter().map(|p: &crate::page::Page<U>| {
                p.clone_buffer()
            }).collect(),
        ]
    }

    #[inline(always)]
    fn to_string_impl(&self) -> String {
        self.chapters
            .borrow()
            .iter()
            .enumerate()
            .map(|(idx, c)| format!("ch{}:{}", idx + 1, c.to_string()))
            .collect::<Vec<String>>()
            .join("\n")
    }

    #[inline(always)]
    fn alloc_impl<T>(&self) -> Option<&mut T> {
        let t_size = size_of::<T>();
        let t_align = align_of::<T>();
        let base_bytes = self.size.base_bytes(t_size, t_align);
        let mut chapters = self.chapters.borrow_mut();
        let chapter = chapters.get_mut(chapter_idx(t_align))?;
        let page_bytes = self.growth.page_bytes(base_bytes, chapter.pages().len());
        let t = chapter.alloc(&self.allocator, t_size, t_align, page_bytes)?.cast();

        unsafe {
            Some(&mut *t)
        }
    }

    #[inline(always)]
    fn dealloc_impl<T>(&self, t: &T) -> bool {
        self.chapters.borrow_mut()[chapter_idx(align_of::<T>())].dealloc((t as *const T).cast())
    }
}

impl<A: BookcaseAllocator, U: Utensil, L> Drop for MultiNotebook<A, U, L> {
    fn drop(&mut self) {
        // Locking is unnecessary since dropping only happens
        // after all references are no longer held.
        for chapter in self.chapters.borrow_mut().iter_mut() {
            chapter.destroy(&self.allocator);
        }
    }
}

pub type PersonalMultiNotebook<A, U> = MultiNotebook<A, U, ()>;

impl<A: BookcaseAllocator, U: Utensil> PersonalMultiNotebook<A, U> {
    pub fn new(
        allocator: A,
        size: SizeStrategy,
        growth: GrowthStrategy,
    ) -> PersonalMultiNotebook<A, U> {
        PersonalMultiNotebook {
            allocator,
            size,
            growth,
            chapters: RefCell::new([
                Chapter::new(),
                Chapter::new(),
                Chapter::new(),
                Chapter::new(),
                Chapter::new(),
            ]),
            lock: (),
        }
    }

    #[cfg(test)]
    pub(crate) fn clone_chapters(&self) -> [Vec<Vec<u8>>; NUM_ALIGNS] {
        self.clone_chapters_impl()
    }
}

impl<A: BookcaseAllocator, U: Utensil> ToString for PersonalMultiNotebook<A, U> {
    fn to_string(&self) -> String {
        self.to_string_impl()
    }
}

impl<A: BookcaseAllocator, U: Utensil> Notebook for PersonalMultiNotebook<A, U> {
    #[inline(always)]
    fn alloc<T>(&self) -> Option<&mut T> {
        self.alloc_impl()
    }

    #[inline(always)]
    fn new<T>(&self, t: T) -> Option<Handle<T>> where Self: Sized {
        let t_ref = self.alloc_impl()?;

        *t_ref = t;
        Some(Handle::new(self, t_ref))
    }

    #[inline(always)]
    fn dealloc<T>(&self, t: &T) -> bool {
        self.dealloc_impl(t)
    }
}

pub type PublicMultiNotebook<A, U> = MultiNotebook<A, U, RwLock<()>>;

impl<A: BookcaseAllocator, U: Utensil> PublicMultiNotebook<A, U> {
    pub fn new(
        allocator: A,
        size: SizeStrategy,
        growth: GrowthStrategy,
    ) -> PublicMultiNotebook<A, U> {
        PublicMultiNotebook {
            lock: RwLock::new(()),
            allocator,
            size,
            growth,
            chapters: RefCell::new([
                Chapter::new(),
                Chapter::new(),
                Chapter::new(),
                Chapter::new(),
                Chapter::new(),
            ]),
        }
    }

    #[cfg(test)]
    pub(crate) fn clone_chapters(&self) -> [Vec<Vec<u8>>; NUM_ALIGNS] {
        let _guard = self.lock.read().unwrap();

        self.clone_chapters_impl()
    }
}

impl<A: BookcaseAllocator, U: Utensil> ToString for PublicMultiNotebook<A, U> {
    fn to_string(&self) -> String {
        let _guard = self.lock.read().unwrap();

        self.to_string_impl()
    }
}

impl<A: BookcaseAllocator, U: Utensil> Notebook for PublicMultiNotebook<A, U> {
    #[inline(always)]
    fn alloc<T>(&self) -> Option<&mut T> {
        let _guard = self.lock.write().unwrap();

        self.alloc_impl()
    }

    #[inline(always)]
    fn new<T>(&self, t: T) -> Option<Handle<T>> where Self: Sized {
        let _guard = self.lock.write().unwrap();
        let t_ref = self.alloc_impl()?;

        *t_ref = t;
        Some(Handle::new(self, t_ref))
    }

    #[inline(always)]
    fn dealloc<T>(&self, t: &T) -> bool {
        let _guard = self.lock.write().unwrap();

        self.dealloc_impl(t)
    }
}

unsafe impl<A: BookcaseAllocator, U: Utensil> Sync for PublicMultiNotebook<A, U> {}

/// Can only allocate one type. This is especially useful for loading a lot of the same data
/// into a cache line to increase cache hits during iteration.
pub struct MonoNotebook<A: BookcaseAllocator, U: Utensil, T, L=()> {
    allocator: A,
    size: SizeStrategy,
    growth: GrowthStrategy,
    chapter: RefCell<Chapter<U>>,
    _pd: PhantomData<T>,
    lock: L,
}

impl<A: BookcaseAllocator, U: Utensil, T, L: Sealed> Sealed for MonoNotebook<A, U, T, L> {}

// These implementations are to reduce duplication.
impl<A: BookcaseAllocator, U: Utensil, T, L> MonoNotebook<A, U, T, L> {
    #[inline(always)]
    fn to_string_impl(&self) -> String {
        self.chapter.borrow().to_string()
    }

    #[inline(always)]
    fn alloc_t_impl(&self) -> Option<&mut T> {
        let t_size = size_of::<T>();
        let t_align = align_of::<T>();
        let base_bytes = self.size.base_bytes(t_size, t_align);
        let mut chapter = self.chapter.borrow_mut();
        let page_bytes = self.growth.page_bytes(base_bytes, chapter.pages().len());
        let t = chapter.alloc(&self.allocator, t_size, t_align, page_bytes)?.cast();

        unsafe {
            Some(&mut *t)
        }
    }

    #[inline(always)]
    fn dealloc_t_impl(&self, t: &T) -> bool {
        self.chapter.borrow_mut().dealloc((t as *const T).cast())
    }
}

impl<A: BookcaseAllocator, U: Utensil, T, L> Drop for MonoNotebook<A, U, T, L> {
    fn drop(&mut self) {
        // Locking is unnecessary since dropping only happens
        // after all references are no longer held.
        self.chapter.borrow_mut().destroy(&self.allocator);
    }
}

pub type PersonalMonoNotebook<A, U, T> = MonoNotebook<A, U, T, ()>;

impl<A: BookcaseAllocator, U: Utensil, T> PersonalMonoNotebook<A, U, T> {
    pub fn new(
        allocator: A,
        size: SizeStrategy,
        growth: GrowthStrategy,
    ) -> PersonalMonoNotebook<A, U, T> {
        PersonalMonoNotebook {
            allocator,
            size,
            growth,
            chapter: RefCell::new(Chapter::new()),
            _pd: PhantomData,
            lock: (),
        }
    }
}

impl<A: BookcaseAllocator, U: Utensil, T> ToString for PersonalMonoNotebook<A, U, T> {
    fn to_string(&self) -> String {
        self.to_string_impl()
    }
}

impl<A: BookcaseAllocator, U: Utensil, T> TypedNotebook<T> for PersonalMonoNotebook<A, U, T> {
    fn alloc_t(&self) -> Option<&mut T> {
        self.alloc_t_impl()
    }

    fn new_t(&self, t: T) -> Option<Handle<T>> where Self: Sized {
        let t_ref = self.alloc_t_impl()?;

        *t_ref = t;
        Some(Handle::new(self, t_ref))
    }

    fn dealloc_t(&self, t: &T) -> bool {
        self.dealloc_t_impl(t)
    }
}

pub type PublicMonoNotebook<A, U, T> = MonoNotebook<A, U, T, RwLock<()>>;

impl<A: BookcaseAllocator, U: Utensil, T> PublicMonoNotebook<A, U, T> {
    pub fn new(
        allocator: A,
        size: SizeStrategy,
        growth: GrowthStrategy,
    ) -> PublicMonoNotebook<A, U, T> {
        PublicMonoNotebook {
            allocator,
            size,
            growth,
            chapter: RefCell::new(Chapter::new()),
            _pd: PhantomData,
            lock: RwLock::new(()),
        }
    }
}

impl<A: BookcaseAllocator, U: Utensil, T> ToString for PublicMonoNotebook<A, U, T> {
    fn to_string(&self) -> String {
        let _guard = self.lock.read().unwrap();

        self.to_string_impl()
    }
}

impl<A: BookcaseAllocator, U: Utensil, T> TypedNotebook<T> for PublicMonoNotebook<A, U, T> {
    fn alloc_t(&self) -> Option<&mut T> {
        let _guard = self.lock.write().unwrap();

        self.alloc_t_impl()
    }

    fn new_t(&self, t: T) -> Option<Handle<T>> where Self: Sized {
        let _guard = self.lock.write().unwrap();
        let t_ref = self.alloc_t_impl()?;

        *t_ref = t;
        Some(Handle::new(self, t_ref))
    }

    fn dealloc_t(&self, t: &T) -> bool {
        let _guard = self.lock.write().unwrap();

        self.dealloc_t_impl(t)
    }
}

unsafe impl<A: BookcaseAllocator, U: Utensil, T> Sync for PublicMonoNotebook<A, U, T> {}

#[inline(always)]
fn chapter_idx(t_align: usize) -> usize {
    t_align.trailing_zeros().min(NUM_ALIGNS as u32 - 1) as usize
}

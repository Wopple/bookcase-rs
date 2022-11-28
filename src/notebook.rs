use std::alloc::Allocator;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::mem::{align_of, size_of};
use std::sync::RwLock;

use crate::{GrowthStrategy, SizeStrategy};
use crate::chapter::Chapter;
use crate::handle::Handle;
use crate::page::{Config, Page};

pub trait Notebook: Send + Sync {
    fn alloc<T>(&self) -> Option<&mut T>;

    /// Zeroes all bytes allocated including padding.
    #[inline]
    fn alloc_zero<T>(&self) -> Option<&mut T> {
        let t_ref = self.alloc()?;

        unsafe {
            (t_ref as *mut T).write_bytes(0, 1);
        }

        Some(t_ref)
    }

    /// Initializes the memory with the given vqlue.
    #[inline]
    fn alloc_init<T>(&self, t: T) -> Option<&mut T> {
        let t_ref = self.alloc()?;

        *t_ref = t;
        Some(t_ref)
    }

    /// Moves a handle to the caller which will drop the value when the handle is dropped.
    #[inline]
    fn new<T>(&self, t: T) -> Option<Handle<T>> where Self: Sized {
        let t_ref = self.alloc_init(t)?;

        Some(Handle::new(self, t_ref))
    }

    fn dealloc<T>(&self, t: &T) -> bool;
}

/// *_t suffix is used so as not to clash with Notebook's interface.
pub trait TypedNotebook<T>: Send + Sync {
    fn alloc_t(&self) -> Option<&mut T>;

    /// Zeroes all bytes allocated including padding.
    #[inline(always)]
    fn alloc_zero_t(&self) -> Option<&mut T> {
        let t_ref = self.alloc_t()?;

        unsafe {
            (t_ref as *mut T).write_bytes(0, 1);
        }

        Some(t_ref)
    }

    /// Initializes the memory with the given vqlue.
    /// Moves a handle to the caller which will drop the value when the handle is dropped.
    #[inline(always)]
    fn alloc_init_t(&self, t: T) -> Option<&mut T> {
        let t_ref = self.alloc_t()?;

        *t_ref = t;
        Some(t_ref)
    }

    #[inline(always)]
    fn new_t(&self, t: T) -> Option<Handle<T>> where Self: Sized {
        let t_ref = self.alloc_init_t(t)?;

        Some(Handle::new(self, t_ref))
    }

    fn dealloc_t(&self, t: &T) -> bool;
}

/// Allows Notebooks to be used as TypedNotebooks.
impl<N: Notebook, T> TypedNotebook<T> for N {
    #[inline(always)]
    fn alloc_t(&self) -> Option<&mut T> {
        self.alloc::<T>()
    }

    #[inline(always)]
    fn alloc_zero_t(&self) -> Option<&mut T> {
        self.alloc_zero::<T>()
    }

    #[inline(always)]
    fn alloc_init_t(&self, t: T) -> Option<&mut T> {
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

/// Can allocate any type. All types will be allocated to their proper alignment. This is
/// especially useful for processing heterogeneous granular data like parsing a JSON string
/// by minimizing the frequency of calling into the operating system for allocation.
pub struct MultiNotebook<A: Allocator, C: Config> {
    lock: RwLock<()>,
    allocator: A,
    size: SizeStrategy,
    growth: GrowthStrategy,
    chapters: RefCell<[Chapter<Page<C>>; NUM_ALIGNS]>,
}

unsafe impl<A: Allocator, C: Config> Send for MultiNotebook<A, C> {}
unsafe impl<A: Allocator, C: Config> Sync for MultiNotebook<A, C> {}

impl<A: Allocator, C: Config> MultiNotebook<A, C> {
    pub fn new(
        allocator: A,
        size: SizeStrategy,
        growth: GrowthStrategy,
    ) -> MultiNotebook<A, C> {
        MultiNotebook {
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
}

impl<A: Allocator, C: Config> Drop for MultiNotebook<A, C> {
    fn drop(&mut self) {
        let _guard = self.lock.write().unwrap();

        for chapter in self.chapters.borrow_mut().iter_mut() {
            chapter.destroy(&self.allocator);
        }
    }
}

impl<A: Allocator, C: Config> ToString for MultiNotebook<A, C> {
    fn to_string(&self) -> String {
        let _guard = self.lock.read().unwrap();

        self.chapters
            .borrow()
            .iter()
            .enumerate()
            .map(|(idx, c)| format!("ch{}:{}", idx + 1, c.to_string()))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl<A: Allocator, C: Config> Notebook for MultiNotebook<A, C> {
    fn alloc<T>(&self) -> Option<&mut T> {
        let t_size = size_of::<T>();
        let t_align = align_of::<T>();
        let base_bytes = self.size.base_bytes(t_size, t_align);
        let idx = chapter_idx(t_align);
        let _guard = self.lock.write().unwrap();
        let mut chapters = self.chapters.borrow_mut();
        let chapter = chapters.get_mut(idx)?;
        let page_bytes = self.growth.page_bytes(base_bytes, chapter.pages().len());
        let t = chapter.alloc(&self.allocator, t_size, t_align, page_bytes)?.cast();

        unsafe {
            Some(&mut *t)
        }
    }

    fn dealloc<T>(&self, t: &T) -> bool {
        let ptr = (t as *const T).cast();
        let idx = chapter_idx(align_of::<T>());
        let _guard = self.lock.write().unwrap();

        self.chapters.borrow_mut()[idx].dealloc(ptr)
    }
}

/// Can only allocate one type. This is especially useful for loading a lot of the same data
/// into a cache line to increase cache hits during iteration.
pub struct MonoNotebook<A: Allocator, C: Config, T> {
    lock: RwLock<()>,
    allocator: A,
    size: SizeStrategy,
    growth: GrowthStrategy,
    chapter: RefCell<Chapter<Page<C>>>,
    _pd: PhantomData<T>
}

unsafe impl<A: Allocator, C: Config, T> Send for MonoNotebook<A, C, T> {}
unsafe impl<A: Allocator, C: Config, T> Sync for MonoNotebook<A, C, T> {}

impl<A: Allocator, C: Config, T> MonoNotebook<A, C, T> {
    pub fn new(
        allocator: A,
        size: SizeStrategy,
        growth: GrowthStrategy,
    ) -> MonoNotebook<A, C, T> {
        MonoNotebook {
            lock: RwLock::new(()),
            allocator,
            size,
            growth,
            chapter: RefCell::new(Chapter::new()),
            _pd: PhantomData,
        }
    }
}

impl<A: Allocator, C: Config, T> ToString for MonoNotebook<A, C, T> {
    fn to_string(&self) -> String {
        let _guard = self.lock.read().unwrap();

        self.chapter.borrow().to_string()
    }
}

impl<A: Allocator, T: Send + Sync, C: Config> TypedNotebook<T> for MonoNotebook<A, C, T> {
    fn alloc_t(&self) -> Option<&mut T> {
        let t_size = size_of::<T>();
        let t_align = align_of::<T>();
        let base_bytes = self.size.base_bytes(t_size, t_align);
        let _guard = self.lock.write().unwrap();
        let mut chapter = self.chapter.borrow_mut();
        let page_bytes = self.growth.page_bytes(base_bytes, chapter.pages().len());
        let t = chapter.alloc(&self.allocator, t_size, t_align, page_bytes)?.cast();

        unsafe {
            Some(&mut *t)
        }
    }

    fn dealloc_t(&self, t: &T) -> bool {
        let ptr = (t as *const T).cast();
        let _guard = self.lock.write().unwrap();

        self.chapter.borrow_mut().dealloc(ptr)
    }
}

#[inline(always)]
fn chapter_idx(align: usize) -> usize {
    match align {
        1 => 0,
        2 => 1,
        4 => 2,
        8 => 3,
        _ => 4,
    }
}

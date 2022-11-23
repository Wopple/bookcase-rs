use std::mem::align_of;
use std::sync::RwLock;

use crate::chapter::{Chapter, ChapterT};
use crate::handle::Handle;
use crate::page::{Config, Page};
use crate::SizeStrategy;

pub trait Notebook: Send + Sync {
    fn alloc<T>(&self) -> Handle<T>;

    /// Zeroes all bytes allocated including padding.
    #[inline]
    fn alloc_zeroed<T>(&self) -> Handle<T> {
        let handle = self.alloc();

        unsafe {
            (handle.t as *mut T).write_bytes(0, 1);
        }

        handle
    }

    #[inline]
    fn alloc_init<T>(&self, t: T) -> Handle<T> {
        let handle = self.alloc();

        *handle.t = t;
        handle
    }

    fn dealloc<T>(&self, t: &T) -> bool;
}

pub trait TypedNotebook<T>: Send + Sync {
    fn alloc_typed(&self) -> Handle<T>;

    /// Zeroes all bytes allocated including padding.
    #[inline]
    fn alloc_zeroed_typed(&self) -> Handle<T> {
        let handle = self.alloc_typed();

        unsafe {
            (handle.t as *mut T).write_bytes(0, 1);
        }

        handle
    }

    #[inline]
    fn alloc_init_typed(&self, t: T) -> Handle<T> {
        let handle = self.alloc_typed();

        *handle.t = t;
        handle
    }

    fn dealloc_typed(&self, t: &T) -> bool;
}

impl<N: Notebook, T> TypedNotebook<T> for N {
    #[inline]
    fn alloc_typed(&self) -> Handle<T> {
        self.alloc::<T>()
    }

    #[inline]
    fn alloc_zeroed_typed(&self) -> Handle<T> {
        self.alloc_zeroed::<T>()
    }

    #[inline]
    fn alloc_init_typed(&self, t: T) -> Handle<T> {
        self.alloc_init::<T>(t)
    }

    #[inline]
    fn dealloc_typed(&self, t: &T) -> bool {
        self.dealloc::<T>(t)
    }
}

/// Can allocate any type. All types will be allocated to their proper alignment. This is
/// especially useful for processing heterogeneous granular data like parsing a JSON string
/// by minimizing the frequency of calling into the operating system for allocation.
pub struct MultiNotebook<C> {
    strategy: SizeStrategy,
    ch1: RwLock<Chapter<Page<u8, C>>>,
    ch2: RwLock<Chapter<Page<u16, C>>>,
    ch3: RwLock<Chapter<Page<u32, C>>>,
    ch4: RwLock<Chapter<Page<u64, C>>>,
    ch5: RwLock<Chapter<Page<u128, C>>>,
}

impl<C> MultiNotebook<C> {
    pub fn new(strategy: SizeStrategy) -> MultiNotebook<C> {
        MultiNotebook {
            strategy,
            ch1: RwLock::new(Chapter::<Page<u8, C>>::new()),
            ch2: RwLock::new(Chapter::<Page<u16, C>>::new()),
            ch3: RwLock::new(Chapter::<Page<u32, C>>::new()),
            ch4: RwLock::new(Chapter::<Page<u64, C>>::new()),
            ch5: RwLock::new(Chapter::<Page<u128, C>>::new()),
        }
    }
}

impl<C: Config> ToString for MultiNotebook<C> {
    fn to_string(&self) -> String {
        format!(
            "ch1: {}
ch2: {}
ch3: {}
ch4: {}
ch5: {}",
            self.ch1.read().unwrap().to_string(),
            self.ch2.read().unwrap().to_string(),
            self.ch3.read().unwrap().to_string(),
            self.ch4.read().unwrap().to_string(),
            self.ch5.read().unwrap().to_string(),
        )
    }
}

impl<C: Config> Notebook for MultiNotebook<C> {
    fn alloc<T>(&self) -> Handle<T> {
        let t = match align_of::<T>() {
            1 => self.ch1.write().unwrap().alloc::<T>(self.strategy),
            2 => self.ch2.write().unwrap().alloc::<T>(self.strategy),
            4 => self.ch3.write().unwrap().alloc::<T>(self.strategy),
            8 => self.ch4.write().unwrap().alloc::<T>(self.strategy),
            16 => self.ch5.write().unwrap().alloc::<T>(self.strategy),
            // should be an invariant
            _ => self.ch1.write().unwrap().alloc::<T>(self.strategy),
        }.cast();

        unsafe {
            Handle::new(self, &mut *t)
        }
    }

    fn dealloc<T>(&self, t: &T) -> bool {
        let ptr = (t as *const T).cast();

        match align_of::<T>() {
            1 => self.ch1.write().unwrap().dealloc(ptr),
            2 => self.ch2.write().unwrap().dealloc(ptr),
            4 => self.ch3.write().unwrap().dealloc(ptr),
            8 => self.ch4.write().unwrap().dealloc(ptr),
            16 => self.ch5.write().unwrap().dealloc(ptr),
            // should be an invariant
            _ => self.ch1.write().unwrap().dealloc(ptr),
        }
    }
}

/// Can only allocate one type. This is especially useful for loading a lot of the same data
/// into a cache line to increase cache hits during iteration.
pub struct MonoNotebook<T, C> {
    size_strategy: SizeStrategy,
    chapter: RwLock<Chapter<Page<T, C>>>,
}

impl<T, C> MonoNotebook<T, C> {
    pub fn new(size_strategy: SizeStrategy) -> MonoNotebook<T, C> {
        MonoNotebook {
            size_strategy,
            chapter: RwLock::new(Chapter::<Page<T, C>>::new()),
        }
    }
}

impl<T: ToString, C> ToString for MonoNotebook<T, C> {
    fn to_string(&self) -> String {
        self.chapter.read().unwrap().to_string()
    }
}

impl<T: Send + Sync, C: Config> TypedNotebook<T> for MonoNotebook<T, C> {
    fn alloc_typed(&self) -> Handle<T> {
        let t = self.chapter.write().unwrap().alloc::<T>(self.size_strategy).cast();

        unsafe {
            Handle::new(self, &mut *t)
        }
    }

    fn dealloc_typed(&self, t: &T) -> bool {
        self.chapter.write().unwrap().dealloc((t as *const T).cast())
    }
}

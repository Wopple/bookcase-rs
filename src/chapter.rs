use std::mem::{align_of, size_of};

use crate::page::{PageT};
use crate::SizeStrategy;

pub(crate) trait ChapterT {
    fn alloc<T>(&mut self, strategy: SizeStrategy) -> *mut u8;
    fn dealloc(&mut self, ptr: *const u8) -> bool;
}

pub(crate) struct Chapter<P> {
    pages: Vec<P>,
}

impl<P> Chapter<P> {
    pub(crate) fn new() -> Chapter<P> {
        Chapter {
            pages: vec![],
        }
    }
}

impl<P: ToString> ToString for Chapter<P> {
    fn to_string(&self) -> String {
        self.pages
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("")
    }
}

impl<P: PageT> ChapterT for Chapter<P> {
    fn alloc<T>(&mut self, strategy: SizeStrategy) -> *mut u8 {
        let alignments = size_of::<T>() / align_of::<T>();

        if let Some(page) = self.pages.iter_mut().rev().find(|p| p.can_alloc(alignments)) {
            page.alloc(alignments)
        } else {
            let mut page = P::new(strategy.alignments::<T>());
            let ptr = page.alloc(alignments);

            self.pages.push(page);
            ptr
        }
    }

    fn dealloc(&mut self, ptr: *const u8) -> bool {
        if let Some(page) = self.pages.iter_mut().find(|p| p.can_dealloc(ptr)) {
            page.dealloc(ptr);
            true
        } else {
            false
        }
    }
}

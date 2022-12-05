use core::alloc::Layout;

use crate::allocator::BookcaseAllocator;
use crate::page::{Page, Utensil};

pub(crate) struct Chapter<C> {
    pages: Vec<Page<C>>,
}

impl<C: Utensil> Chapter<C> {
    pub(crate) fn new() -> Chapter<C> {
        Chapter { pages: vec![] }
    }

    pub(crate) fn pages(&self) -> &[Page<C>] {
        &self.pages
    }

    pub(crate) fn alloc(
        &mut self,
        allocator: &dyn BookcaseAllocator,
        t_size: usize,
        t_align: usize,
        page_bytes: usize,
    ) -> Option<*mut u8> {
        if let Some(page) = self.pages.iter_mut().rev().find(|p| p.can_alloc(t_size)) {
            Some(page.alloc(t_size))
        } else {
            let layout = Layout::from_size_align(page_bytes, t_align).ok()?;
            let mut page = Page::create(layout, allocator)?;
            let ptr = page.alloc(t_size);

            self.pages.push(page);
            Some(ptr)
        }
    }

    pub(crate) fn dealloc(&mut self, ptr: *const u8) -> bool {
        if let Some(page) = self.pages.iter_mut().find(|p| p.can_dealloc(ptr)) {
            page.dealloc(ptr);
            true
        } else {
            false
        }
    }

    pub(crate) fn destroy(&mut self, allocator: &dyn BookcaseAllocator) {
        for page in self.pages.iter_mut() {
            page.destroy(allocator)
        }
    }
}

impl<C> ToString for Chapter<C> {
    fn to_string(&self) -> String {
        self.pages
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("")
    }
}

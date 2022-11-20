use std::fmt;
use std::ops::{Deref, DerefMut};

use crate::TypedNotebook;

/// The handle is useful for cleaning up resources the type owns outside of the notebook. For
/// example, String stores its characters on the heap, dropping the notebook does not clean
/// up those bytes on the heap. When the handle goes out of scope, it gets dropped and then
/// calls the type's drop method in turn. The handle also allows for automatic deallocation
/// from the notebook using the same mechanism.
pub struct Handle<'book, T> {
    pub(crate) notebook: &'book dyn TypedNotebook<T>,
    pub(crate) t: &'book mut T,
}

impl<'book, T> Handle<'book, T> {
    pub fn new(notebook: &'book dyn TypedNotebook<T>, t: &'book mut T) -> Handle<'book, T> {
        Handle { notebook, t }
    }
}

impl<'book, T: fmt::Debug> fmt::Debug for Handle<'book, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Handle").field("t", self.t).finish()
    }
}

impl<'book, T> Deref for Handle<'book, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.t
    }
}

impl<'book, T> DerefMut for Handle<'book, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.t
    }
}

impl<'book, T> Drop for Handle<'book, T> {
    fn drop(&mut self) {
        unsafe {
            // cleans up any resources the type owns outside of the notebook
            (self.t as *mut T).drop_in_place();
        }

        self.notebook.dealloc_typed(self.t);
    }
}

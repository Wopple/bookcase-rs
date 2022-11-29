#![feature(allocator_api)]
#![feature(ptr_internals)]

#[cfg(not(test))]
::bookcase_macros::assert_release_channel!();

pub use notebook::*;
pub use strategy::*;

pub(crate) mod chapter;
pub(crate) mod handle;
pub(crate) mod notebook;
pub(crate) mod page;
pub(crate) mod strategy;

#[cfg(test)]
mod tests {
    use std::alloc;
    use crate::{GrowthStrategy, MonoNotebook, MultiNotebook, Notebook, SizeStrategy, TypedNotebook};
    use crate::page::BumpConfig;

    #[derive(Debug, Eq, PartialEq)]
    struct S1 {
        a: usize,
        b: isize,
    }

    #[test]
    fn test() {
        {
            let notebook = MultiNotebook::<_, BumpConfig>::new(
                alloc::Global,
                SizeStrategy::WordsPerPage(4),
                GrowthStrategy::Constant,
            );

            let i32_value = notebook.alloc::<i32>().unwrap();
            *i32_value = 0x0302i32;
            notebook.alloc_init(0i32);
            notebook.alloc_init(0i32);
            notebook.alloc_init(0i32);
            notebook.alloc_init(0i32);
            notebook.alloc_init(0i32);
            notebook.alloc_init(0i32);
            notebook.alloc_init(0i32);

            let typed: &dyn TypedNotebook<usize> = &notebook;
            typed.alloc_init_t(4usize);
            typed.alloc_init_t(5usize);
            typed.alloc_init_t(6usize);
            typed.alloc_init_t(7usize);

            assert_eq!(
                [
                    vec![],
                    vec![],
                    vec![
                        vec![2u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8],
                    ],
                    vec![
                        vec![4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8],
                    ],
                    vec![],
                ],
                notebook.clone_chapters(),
            );

            assert_eq!(770, *i32_value);

            let str_value = notebook.alloc_init(String::from("1u8"));

            assert_eq!("1u8", str_value.unwrap());
        }

        {
            let typed_notebook = MonoNotebook::<_, BumpConfig, S1>::new(
                alloc::Global,
                SizeStrategy::ItemsPerPage(4),
                GrowthStrategy::Constant,
            );

            let s1 = typed_notebook.alloc_init_t(S1 {
                a: 0x01020304,
                b: -0x04030201,
            });

            assert_eq!(S1 { a: 16909060, b: -67305985 }, *s1.unwrap());
        }
    }
}

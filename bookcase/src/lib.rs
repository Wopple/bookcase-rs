#![cfg_attr(feature = "allocator_api", feature(allocator_api))]

pub use notebook::*;
pub use strategy::*;

#[cfg(not(test))]
::bookcase_alloc_macros::assert_release_channel!();

pub(crate) mod allocator;
pub(crate) mod chapter;
pub(crate) mod error;
pub(crate) mod handle;
pub(crate) mod notebook;
pub(crate) mod page;
pub(crate) mod seal;
pub(crate) mod strategy;

#[cfg(test)]
mod tests {
    #[cfg(feature = "allocator_api")]
    use std::alloc::Global as Allocator;

    use crate::*;
    #[cfg(not(feature = "allocator_api"))]
    use crate::allocator::stable_allocator::StdAllocator as Allocator;
    use crate::page::*;

    #[derive(Debug, Eq, PartialEq)]
    struct TestStruct {
        a: usize,
        b: isize,
    }

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    #[test]
    fn test() {
        assert_send::<PersonalMultiNotebook::<Allocator, Pen>>();
        assert_send::<PublicMultiNotebook::<Allocator, Pen>>();
        assert_send::<PersonalMonoNotebook::<Allocator, Pen, usize>>();
        assert_send::<PublicMonoNotebook::<Allocator, Pen, usize>>();

        assert_sync::<PublicMultiNotebook::<Allocator, Pen>>();
        assert_sync::<PublicMonoNotebook::<Allocator, Pen, usize>>();

        {
            let notebook = MultiNotebook::<_, Pen>::new(
                Allocator,
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
            let typed_notebook = PersonalMonoNotebook::<_, Pen, TestStruct>::new(
                Allocator,
                SizeStrategy::ItemsPerPage(4),
                GrowthStrategy::Constant,
            );

            let s1 = typed_notebook.alloc_init_t(TestStruct {
                a: 0x01020304,
                b: -0x04030201,
            });

            assert_eq!(TestStruct { a: 16909060, b: -67305985 }, *s1.unwrap());
        }
    }
}

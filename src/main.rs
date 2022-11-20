#![feature(allocator_api)]
#![feature(dropck_eyepatch)]
#![feature(ptr_internals)]

use crate::notebook::*;
use crate::page::BumpConfig;
use crate::strategy::*;

mod chapter;
mod handle;
mod notebook;
mod page;
mod rust_internals;
mod strategy;

#[derive(Debug)]
struct S1 {
    a: usize,
    b: isize,
}

fn main() {
    let mut notebook = MultiNotebook::<BumpConfig>::new(SizeStrategy::WordsPerPage(4));

    notebook.alloc_init(1u8);
    let v1 = notebook.alloc_init(String::from("1u8"));
    let v2 = notebook.alloc_init(0x0302i32);

    {
        let s1 = notebook.alloc_init(S1 {
            a: 0x01020304,
            b: -0x04030201,
        });
        println!("{:?}", s1);
    }

    let typed: &dyn TypedNotebook<usize> = &notebook;
    typed.alloc_init_typed(7usize);
    typed.alloc_init_typed(7usize);
    typed.alloc_init_typed(7usize);

    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{}", notebook.to_string());
}

extern crate arc_swap;
extern crate bitvec;
extern crate init_array;
extern crate static_assertions;
extern crate triomphe;

#[cfg(test)]
extern crate assert_matches;

use static_assertions::{const_assert, const_assert_eq};

pub use arc_swap::RefCnt;

mod utility;

mod value;

pub mod simple_table;
pub mod swiss_like_table;

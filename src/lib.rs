//! This implementation is partly inspired from swisstable

extern crate arc_swap;
extern crate bitvec;
extern crate init_array;
extern crate static_assertions;

use static_assertions::{const_assert, const_assert_eq};

pub use arc_swap::RefCnt;

#[cfg(test)]
extern crate assert_matches;

mod utility;

mod value;

pub use value::SlotIndex;

mod slot;

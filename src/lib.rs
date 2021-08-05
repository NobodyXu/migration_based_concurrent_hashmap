//! This implementation is partly inspired from swisstable

extern crate arc_swap;
extern crate static_assertions;

use static_assertions::{const_assert, const_assert_eq};

#[cfg(test)]
extern crate assert_matches;

mod utility;

pub use arc_swap::RefCnt;

mod value;

pub use value::SlotIndex;
use value::Value;

mod slot;

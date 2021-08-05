extern crate arc_swap;
extern crate atomic;
extern crate static_assertions;

use static_assertions::const_assert;

#[cfg(test)]
extern crate assert_matches;

pub use arc_swap::RefCnt;

mod value;

pub use value::SlotIndex;
use value::Value;

mod slot;

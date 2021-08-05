extern crate arc_swap;

#[cfg(test)]
extern crate assert_matches;

pub use arc_swap::RefCnt;

mod value;

pub use value::SlotIndex;
use value::Value;

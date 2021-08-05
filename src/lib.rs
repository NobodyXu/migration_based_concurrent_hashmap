extern crate arc_swap;

pub use arc_swap::RefCnt;

mod value;

pub use value::SlotIndex;
use value::Value;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

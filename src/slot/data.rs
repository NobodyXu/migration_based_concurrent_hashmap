use arc_swap::{ArcSwapAny, RefCnt};
use init_array::init_array;

use crate::value::Value;
use super::metadata::BLOCKSIZE;

pub struct DataBlock<T: RefCnt> {
    inner: [ArcSwapAny<Value<T>>; BLOCKSIZE]
}
impl<T: RefCnt> Default for DataBlock<T> {
    fn default() -> Self {
        Self {
            inner: init_array(|_| Default::default()),
        }
    }
}
impl<T: RefCnt> DataBlock<T> {
    #[inline(always)]
    pub fn get(&self, index: u8) -> &ArcSwapAny<Value<T>> {
        &self.inner[index as usize]
    }
}

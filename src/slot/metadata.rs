use core::sync::atomic::AtomicU64;
use core::mem::{size_of};

use crate::utility::*;

const EMPTY: u8 = 1 << 7;        // 0b10000000
const DELETED: u8 = u8::MAX - 1; // 0b11111110

pub const BLOCKSIZE: usize = 8; // in bytes
pub type Atomic = AtomicU64;
pub type BitMask = bitvec::array::BitArray<bitvec::order::LocalBits, [u8; 1]>;

crate::const_assert_eq!(size_of::<Atomic>(), 8);
    crate::const_assert_eq!(size_of::<BitMask>() * 8, BLOCKSIZE);

pub struct AtomicMetaBlock {
    inner: Atomic,
}
impl Default for AtomicMetaBlock {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
impl AtomicMetaBlock {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            inner: Atomic::new(u64::from_ne_bytes([EMPTY; BLOCKSIZE])),
        }
    }

    #[inline(always)]
    pub fn load(&self) -> MetaBlockValue {
        MetaBlockValue::new(self.inner.load(R_ORD))
    }

    /// Return a new up-to-date version of the `MetaBlockValue` on failure.
    pub fn compare_exchange_weak(&self, value: MetaBlockValue)
        -> Result<(), MetaBlockValue>
    {
        match self.inner.compare_exchange_weak(
            value.get_old_val(),
            value.get_new_val(),
            RW_ORD,
            R_ORD
        ) {
            Ok(_) => Ok(()),
            Err(val) => Err(MetaBlockValue::new(val))
        }
    }
}

pub struct MetaBlockValue {
    bytes: [u8; BLOCKSIZE],
    old_bytes: [u8; BLOCKSIZE],
}
impl MetaBlockValue {
    #[inline(always)]
    const fn new(val: u64) -> Self {
        let bytes = val.to_ne_bytes();
        Self {
            bytes,
            old_bytes: bytes,
        }
    }

    fn get_old_val(&self) -> u64 {
        u64::from_ne_bytes(self.old_bytes)
    }

    fn get_new_val(&self) -> u64 {
        u64::from_ne_bytes(self.bytes)
    }

    pub fn find_empty(&self) -> BitMask {
        let mut bitmask = BitMask::zeroed();

        for (byte, mut bitflag) in self.bytes.iter().zip(&mut bitmask) {
            if *byte == EMPTY {
                *bitflag = true;
            }
        }

        bitmask
    }

    ///  * `hash` - should be calculated using `super::H2`
    pub fn find_hash_matched(&self, hash: u8) -> BitMask {
        let mut bitmask = BitMask::zeroed();

        for (byte, mut bitflag) in self.bytes.iter().zip(&mut bitmask) {
            if *byte == hash {
                *bitflag = true;
            }
        }

        bitmask
    }

    ///  * `index` - `index < BLOCKSIZE`
    pub fn insert(&mut self, index: u8, hash: u8) {
        self.bytes[index as usize] = hash;
    }

    ///  * `index` - `index < BLOCKSIZE`
    pub fn remove(&mut self, index: u8) {
        self.bytes[index as usize] = DELETED;
    }
}

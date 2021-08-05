use core::ptr;

use crate::RefCnt;

/// SlotIndex assumes the first 256 bytes of the address space are unused.
#[derive(Copy, Clone, Debug)]
pub struct SlotIndex(u8);
impl SlotIndex {
    pub const fn new(index: u8) -> Option<Self> {
        if index < 2 || index == u8::MAX {
            None
        } else {
            Some(Self(index))
        }
    }

    /// # Safety
    ///
    ///  - `index` - Must be greater than `1` and less than or equal to `Self::max()`.
    pub const unsafe fn new_unchecked(index: u8) -> Self {
        Self(index)
    }

    const fn max() -> usize {
        (u8::MAX - 1) as usize
    }

    fn to_ptr(self) -> *mut () {
        self.0 as *mut ()
    }
}

#[derive(Clone)]
pub enum Value<T: RefCnt> {
    None,
    Migrated(SlotIndex),
    Some(T),
}
impl<T: RefCnt> Value<T> {
    fn map_ptr(ptr: *mut T::Base) -> *mut T::Base {
        if ptr.is_null() {
            (SlotIndex::max() + 1) as *mut T::Base
        } else {
            ptr
        }
    }
}
unsafe impl<T: RefCnt> RefCnt for Value<T> {
    type Base = T::Base;

    fn into_ptr(me: Self) -> *mut Self::Base {
        match me {
            Self::None => ptr::null_mut(),
            Self::Migrated(index) => index.to_ptr() as *mut Self::Base,
            Self::Some(value) => Self::map_ptr(RefCnt::into_ptr(value)),
        }
    }

    fn as_ptr(me: &Self) -> *mut Self::Base {
        match me {
            Self::None => ptr::null_mut(),
            Self::Migrated(index) => index.to_ptr() as *mut Self::Base,
            Self::Some(value) => Self::map_ptr(RefCnt::as_ptr(value)),
        }
    }

    unsafe fn from_ptr(ptr: *const Self::Base) -> Self {
        if ptr.is_null() {
            Self::None
        } else if (ptr as usize) <= SlotIndex::max() {
            Self::Migrated(SlotIndex::new_unchecked(ptr as u8))
        } else if (ptr as usize) == (SlotIndex::max() + 1) {
            Self::Some(T::from_ptr(ptr::null()))
        } else {
            Self::Some(T::from_ptr(ptr))
        }
    }
}

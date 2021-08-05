use core::ptr;

use crate::RefCnt;

/// SlotIndex assumes the first 256 bytes of the address space are unused.
#[derive(Copy, Clone, Debug)]
pub struct SlotIndex(u8);
impl SlotIndex {
    #[inline(always)]
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
    #[inline(always)]
    pub const unsafe fn new_unchecked(index: u8) -> Self {
        Self(index)
    }

    #[inline(always)]
    pub const fn to_raw(self) -> u8 {
        self.0
    }

    #[inline(always)]
    const fn max() -> usize {
        (u8::MAX - 1) as usize
    }

    #[inline(always)]
    const fn to_ptr(self) -> *mut () {
        self.0 as *mut ()
    }
}

#[derive(Clone, Debug)]
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

#[cfg(test)]
mod tests {
    use core::ptr;
    use assert_matches::assert_matches;

    use super::{SlotIndex, RefCnt};

    type Arc = std::sync::Arc<()>;
    type Value = super::Value<Option<Arc>>;

    #[test]
    fn test_none() {
        let value = Value::None;

        assert_eq!(ptr::null_mut(), RefCnt::as_ptr(&value));
        assert_eq!(ptr::null_mut(), RefCnt::inc(&value));
        unsafe {
            <Value as RefCnt>::dec(ptr::null_mut());
        }
        assert_eq!(ptr::null_mut(), RefCnt::into_ptr(value));

        assert_matches!(
            unsafe { RefCnt::from_ptr(ptr::null()) },
            Value::None
        );
    }

    #[test]
    fn test_migrated() {
        for i in 2..(u8::MAX) {
            let index = SlotIndex::new(i).unwrap();
            assert_eq!(i as usize, index.to_ptr() as usize);

            let value = Value::Migrated(index);
            assert_eq!(i as usize, RefCnt::as_ptr(&value) as usize);
            assert_eq!(i as usize, RefCnt::inc(&value) as usize);
            unsafe {
                <Value as RefCnt>::dec(index.to_ptr());
            }
            assert_eq!(i as usize, RefCnt::into_ptr(value) as usize);

            assert_matches!(
                unsafe { RefCnt::from_ptr(i as usize as *const _) },
                Value::Migrated(index) if index.to_raw() == i
            );
        }
    }

    #[test]
    fn test_no_arc() {
        let value = Value::Some(None);

        assert_eq!(SlotIndex::max() + 1, RefCnt::as_ptr(&value) as usize);
        assert_eq!(SlotIndex::max() + 1, RefCnt::inc(&value) as usize);
        unsafe {
            <Value as RefCnt>::dec((SlotIndex::max() + 1) as *const ());
        }
        assert_eq!(SlotIndex::max() + 1, RefCnt::into_ptr(value) as usize);

        assert_matches!(
            unsafe { RefCnt::from_ptr((SlotIndex::max() + 1) as *const ()) },
            Value::Some(None)
        );
    }
}

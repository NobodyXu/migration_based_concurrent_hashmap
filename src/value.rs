use core::ptr;

use crate::RefCnt;

/// Value assumes addresses from `0x0` to `0x3` are unused.
#[derive(Clone, Debug)]
pub enum Value<T: RefCnt> {
    None,
    Deleted,
    Migrated,
    Some(T),
}
impl<T: RefCnt> Default for Value<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::None
    }
}
impl<T: RefCnt> Value<T> {
    #[inline(always)]
    fn map_ptr(ptr: *mut T::Base) -> *mut T::Base {
        if ptr.is_null() {
            3 as *mut T::Base
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
            Self::Deleted => 1 as *mut Self::Base,
            Self::Migrated => 2 as *mut Self::Base,
            Self::Some(value) => Self::map_ptr(RefCnt::into_ptr(value)),
        }
    }

    fn as_ptr(me: &Self) -> *mut Self::Base {
        match me {
            Self::None => ptr::null_mut(),
            Self::Deleted => 1 as *mut Self::Base,
            Self::Migrated => 2 as *mut Self::Base,
            Self::Some(value) => Self::map_ptr(RefCnt::as_ptr(value)),
        }
    }

    unsafe fn from_ptr(ptr: *const Self::Base) -> Self {
        if ptr.is_null() {
            Self::None
        } else if (ptr as usize) == 1 {
            Self::Deleted
        } else if (ptr as usize) == 2 {
            Self::Migrated
        } else if (ptr as usize) == 3 {
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

    use super::RefCnt;

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
    fn test_deleted() {
        let value = Value::Deleted;
        let ptr = 1 as *mut _;

        assert_eq!(ptr, RefCnt::as_ptr(&value));
        assert_eq!(ptr, RefCnt::inc(&value));
        unsafe {
            <Value as RefCnt>::dec(ptr);
        }
        assert_eq!(ptr, RefCnt::into_ptr(value));

        assert_matches!(
            unsafe { RefCnt::from_ptr(ptr) },
            Value::Deleted
        );
    }

    #[test]
    fn test_migrated() {
        let value = Value::Migrated;
        assert_eq!(2 as usize, RefCnt::as_ptr(&value) as usize);
        assert_eq!(2 as usize, RefCnt::inc(&value) as usize);
        unsafe {
            <Value as RefCnt>::dec(2 as *const _);
        }
        assert_eq!(2 as usize, RefCnt::into_ptr(value) as usize);

        assert_matches!(
            unsafe { RefCnt::from_ptr(2 as *const _) },
            Value::Migrated
        );
    }

    #[test]
    fn test_no_arc() {
        let value = Value::Some(None);

        assert_eq!(3, RefCnt::as_ptr(&value) as usize);
        assert_eq!(3, RefCnt::inc(&value) as usize);
        unsafe {
            <Value as RefCnt>::dec(3 as *const ());
        }
        assert_eq!(3, RefCnt::into_ptr(value) as usize);

        assert_matches!(
            unsafe { RefCnt::from_ptr(3 as *const ()) },
            Value::Some(None)
        );
    }

    #[test]
    fn test_arc() {
        let arc = Arc::new(());
        let ptr = Arc::as_ptr(&arc);
        let value = Value::Some(Some(arc));

        assert_eq!(ptr, RefCnt::as_ptr(&value));
        assert_eq!(ptr, RefCnt::inc(&value));
        unsafe {
            <Value as RefCnt>::dec(ptr);
        }
        assert_matches!(
            unsafe { RefCnt::from_ptr(ptr) },
            Value::Some(Some(arc)) if Arc::as_ptr(&arc) == ptr
        );

        assert_eq!(ptr, RefCnt::into_ptr(value));
    }
}

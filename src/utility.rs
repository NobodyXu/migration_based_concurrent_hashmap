use core::sync::atomic::Ordering;

pub const RW_ORD: Ordering = Ordering::AcqRel;
pub const R_ORD: Ordering = Ordering::Acquire;
pub const W_ORD: Ordering = Ordering::Release;

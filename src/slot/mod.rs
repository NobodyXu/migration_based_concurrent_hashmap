#[inline(always)]
const fn H1(hash: usize) -> usize {
    hash >> 7
}

#[inline(always)]
const fn H2(hash: usize) -> u8 {
    (hash as u8) & 127
}

/// Insertion updates the metadata before the actual data;
/// Removal updates the actual data before the metadata.
///
/// So locating the elements requires null checking.
///
/// Also, deleted entries cannot be reused.
/// Otherwise, there will be data race.
mod metadata;
mod data;

#[inline(always)]
const fn extract_bits_mask(len: usize) -> u64 {
    debug_assert!(len <= 64);
    u64::MAX >> (64 - len)
}

#[inline(always)]
const fn extract_bits_shifted_mask(offset: usize, len: usize) -> u64 {
    extract_bits_mask(len) << offset
}

/// extracts some bits from a value
#[inline(always)]
pub const fn extract_bits(value: u64, offset: usize, len: usize) -> u64 {
    let mask = extract_bits_mask(len);
    (value >> offset) & mask
}

/// extracts some bits (mask only, no shift) from a value
#[inline(always)]
pub const fn extract_bits_noshift(value: u64, offset: usize, len: usize) -> u64 {
    value & extract_bits_shifted_mask(offset, len)
}
/// returns a new value with the specified bit range modified to the new value
#[inline(always)]
pub const fn modify_bits(value: u64, offset: usize, len: usize, new_value: u64) -> u64 {
    let shifted_mask = extract_bits_shifted_mask(offset, len);

    let without_original_bits = value & (!shifted_mask);
    let shifted_new_value = new_value << offset;
    without_original_bits | shifted_new_value
}

pub const fn const_array_max_u64(array: &[u64]) -> u64 {
    let mut maybe_max = None;
    use const_for::const_for;
    const_for!(i in 0..array.len() => {
        let cur = array[i];
        match maybe_max {
            Some(max) => {
                if cur > max {
                    maybe_max = Some(cur)
                }
            },
            None => {
                maybe_max = Some(cur)
            }
        }
    });
    maybe_max.unwrap()
}

pub const fn const_array_min_u64(array: &[u64]) -> u64 {
    let mut maybe_min = None;
    use const_for::const_for;
    const_for!(i in 0..array.len() => {
        let cur = array[i];
        match maybe_min {
            Some(min) => {
                if cur < min {
                    maybe_min = Some(cur)
                }
            },
            None => {
                maybe_min = Some(cur)
            }
        }
    });
    maybe_min.unwrap()
}

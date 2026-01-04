#[inline(always)]
const fn extract_bits_mask(len: usize) -> u64 {
    (1u64 << len).wrapping_sub(1)
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
    let mask = extract_bits_mask(len);
    let shifted_mask = mask << offset;
    value & shifted_mask
}
/// returns a new value with the specified bit range modified to the new value
#[inline(always)]
pub const fn modify_bits(value: u64, offset: usize, len: usize, new_value: u64) -> u64 {
    let shifted_mask = extract_bits_shifted_mask(offset, len);

    let without_original_bits = value & (!shifted_mask);
    let shifted_new_value = new_value << offset;
    without_original_bits | shifted_new_value
}

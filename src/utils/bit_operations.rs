
/// Gets the bit at the specified position.
pub fn get_bit(x: u32, i: u32) -> u32 {
    ((x >> i) & 1) as u32
}

/// Reverses the bits of a number up to the specified number of bits.
/// # Parameters
/// - `x`: The number to reverse the bits of.
/// - `n`: The number of bits to reverse.
/// 
/// # Returns
/// The number with the bits reversed.
pub fn reverse_bits(x: u32, n: u32) -> u32 {
    let mut rev = 0;
    for i in 0..n {
        rev = rev << 1;
        rev += get_bit(x, i);
    }
    rev
}

/// Calculates the number of bits between the specified start and end positions.
/// 
/// # Parameters
/// - `start`: The starting bit position.
/// - `end`: The ending bit position.
/// 
/// # Returns
/// The total number of bits from the start position to the end position, inclusive.
pub fn isolate_bits(value: u64, start: u8, end: u8) -> u64 {
    assert!(start <= end && end < 64);
    let num_bits = end - start + 1;
    let shifted = value >> start;
    let mask = (1 << num_bits) - 1;
    shifted & mask
}
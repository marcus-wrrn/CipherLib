
pub fn get_bit(x: u32, i: u32) -> u32 {
    ((x >> i) & 1) as u32
}

pub fn reverse_bits(x: u32, n: u32) -> u32 {
    let mut rev = 0;
    for i in 0..n {
        rev = rev << 1;
        rev += get_bit(x, i);
    }
    rev
}
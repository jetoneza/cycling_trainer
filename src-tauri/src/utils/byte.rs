pub fn combine_u8_to_u16(first: u8, second: u8) -> u16 {
    (first as u16) | (second as u16) << 8
}

pub fn combine_u8_to_u32(first: u8, second: u8, third: u8) -> u32 {
    (first as u32) | (second as u32) << 8 | (third as u32) << 16
}

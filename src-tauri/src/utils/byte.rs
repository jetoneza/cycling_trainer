pub fn combine_u8_to_u16(first: u8, second: u8) -> u16 {
    (first as u16) | (second as u16) << 8
}

pub fn combine_u8_to_u32(first: u8, second: u8, third: u8) -> u32 {
    (first as u32) | (second as u32) << 8 | (third as u32) << 16
}

pub fn convert_i16_to_u8(num: i16) -> [u8; 2] {
    let i16_num = num as i16;

    [(i16_num & 0xFF) as u8, ((i16_num >> 8) & 0xFF) as u8]
}

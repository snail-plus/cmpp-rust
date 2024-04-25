pub fn u32_to_byte_array(value: u32) -> [u8; 4] {
    let bytes = [
        (value >> 24) as u8,
        (value >> 16) as u8,
        (value >> 8) as u8,
        value as u8,
    ];
    bytes
}
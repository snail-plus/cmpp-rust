pub fn u32_to_byte_array(value: u32) -> [u8; 4] {
    let bytes = [
        (value >> 24) as u8,
        (value >> 16) as u8,
        (value >> 8) as u8,
        value as u8,
    ];
    bytes
}

pub fn u64_to_byte_array(value: u64) -> [u8; 8] {
    let bytes = value.to_le_bytes(); // 转换为小端字节序的字节数组
    return bytes;
}
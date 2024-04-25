use std::string::FromUtf16Error;

pub fn octet_string(s: String, fixed_length: usize) -> String {
    let length = s.len();
    if length == fixed_length {
        return s;
    }

    if length > fixed_length {
        let truncated = &s[..fixed_length];
        return String::from(truncated);
    }

    let binding = " ".repeat(fixed_length - length);
    let padding = binding.as_str();
    s + padding
}


pub fn oct_string(v: Vec<u8>) -> String {
    String::from_utf8(v).unwrap().replace("\0", "")
}

// 将假定为大端序UTF-16（即UCS-2）编码的字节切片转换为UTF-8字符串
pub fn ucs2_to_utf8(ucs2_bytes: &[u8]) -> Result<String, FromUtf16Error> {
    // 确保字节数组的长度是 2 的倍数，因为每个 UCS-2 字符是 2 个字节
    if ucs2_bytes.len() % 2 != 0 {
        panic!("UCS-2 byte array length must be even");
    }

    // 将字节数组解码为 u16 切片
    let utf16_chars: Vec<u16> = ucs2_bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap()))
        .collect();

    // 将 UTF-16 切片转换为 UTF-8 字符串
    String::from_utf16(&utf16_chars)
}


#[cfg(test)]
mod tests {
    use crate::util::str::octet_string;

    #[test]
    fn test_octet_string() {
        let c = octet_string(String::from("a"), 3);
        assert_eq!(c, "a  ")
    }
}
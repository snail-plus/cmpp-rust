use std::fmt::Error;
use std::str;

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
pub fn ucs2_to_utf8(in_bytes: Vec<u8>) -> Result<String, String> {
    // 确保字节数组长度是u16的倍数，因为每个UCS-2字符由两个字节组成
    if in_bytes.len() % 2 != 0 {
        return Err("Invalid UCS-2 byte sequence: length is not a multiple of 2".to_string());
    }

    let mut utf8_bytes = Vec::new();
    for chunk in in_bytes.chunks(2) {
        let u16_val = u16::from_be_bytes(chunk.try_into().unwrap());

        // 尝试将u16转换为char
        match char::try_from(u16_val) {
            Ok(c) => {
                // 如果转换成功，将char编码为UTF-8并添加到utf8_bytes中
                utf8_bytes.extend(c.encode_utf8(&mut [0; 4]).iter().cloned());
            }
            Err(_) => {
                // 如果u16值不是一个有效的Unicode字符，返回错误
                return Err("Invalid UCS-2 character".to_string());
            }
        }
    }

    // 将utf8_bytes转换为UTF-8编码的字符串
    let utf8_str = String::from_utf8(utf8_bytes)
        .map_err(|e| format!("Failed to create UTF-8 string: {}", e))?;

    Ok(utf8_str)
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
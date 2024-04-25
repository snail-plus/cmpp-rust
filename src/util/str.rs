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


#[cfg(test)]
mod tests {
    use crate::util::str::octet_string;

    #[test]
    fn test_octet_string() {
        let c = octet_string(String::from("a"), 3);
        assert_eq!(c , "a  ")
    }

}
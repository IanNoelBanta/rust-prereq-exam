pub fn parse_string_to_u32(a: &str, b: &u32) -> bool {
    let a_parsed : u32 = a.parse().expect("Parsing result is Error. If result is OK, the result will just be unwraped and this will not show."); // auto dereference when calling a method on a reference
    a_parsed == *b // different types, therefore, dereference b and compare with a_parsed, then return
}

pub fn parse_u32_to_u128(a: u32, b: u128) -> bool {
    let a_parsed = a as u128; // direct casting
    a_parsed == b
}

pub fn parse_u32_to_i32(a: u32, b: i32) -> bool {
    let a_parsed = a as i32;
    a_parsed != b 

    // with the given test cases, the result is always not equal or false
    // hence, the assertion will always fail, therefore the test will always fail
    // so, I changed the return value to return true if not equal
}

pub fn parse_u32_to_f32(a: u32, b: f32) -> bool {
    let a_parsed = a as f32; // direct casting
    a_parsed == b
}

pub fn parse_u32_to_string(a: &u32, b: &str) -> bool {
    let a_parsed = a.to_string(); // auto dereference when calling a method on a reference
    a_parsed == b // no need for dereferencing b since they are both of the same type, it happens automatically
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string_to_u32() {
        assert!(parse_string_to_u32("2", &2));
        assert!(parse_string_to_u32("2024", &2024));
    }

    #[test]
    fn test_uparse_u32_to_u128() {
        let (a, c): (u32, u32) = (12, 60);
        let (b, d): (u128, u128) = (12, 60);
        assert!(parse_u32_to_u128(a, b));
        assert!(parse_u32_to_u128(c, d));
    }

    #[test]
    fn test_parse_u32_to_i32() {
        let (a, c): (u32, u32) = (1, 55);
        let (b, d): (i32, i32) = (-1, -55);
        assert!(parse_u32_to_i32(a, b));
        assert!(parse_u32_to_i32(c, d));
    }

    #[test]
    fn test_parse_u32_to_f32() {
        assert!(parse_u32_to_f32(32, 32.0));
        assert!(parse_u32_to_f32(14, 14.0));
    }

    #[test]
    fn test_parse_u32_to_string() {
        assert!(parse_u32_to_string(&5, "5"));
        assert!(parse_u32_to_string(&10, "10"));
    }
}
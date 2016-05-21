extern crate libc;

use std::ffi::{CString, NulError};

mod ffi {
    use libc::{c_char, c_uint};
    extern {
        pub fn levenshtein(a: *const c_char, b: *const c_char) -> c_uint;
    }
}

pub fn levenshtein(a: &str, b: &str) -> Result<u32, NulError> {
    let a = try!(CString::new(a));
    let b = try!(CString::new(b));

    Ok(unsafe{ ffi::levenshtein(a.as_ptr(), b.as_ptr()) })
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_strings() {
        assert_eq!(::levenshtein("", ""), Ok(0));
    }
    #[test]
    fn empty_string() {
        assert_eq!(::levenshtein("abc", ""), Ok(3));
        assert_eq!(::levenshtein("", "abc"), Ok(3));
    }
    #[test]
    fn both_same() {
        assert_eq!(::levenshtein("abc", "abc"), Ok(0));
    }
    #[test]
    #[should_panic(expect = "NulError")]
    fn null_char1() {
         ::levenshtein("\0", "abc").unwrap();
    }
    #[test]
    #[should_panic(expect = "NulError")]
    fn null_char2() {
        ::levenshtein("abv", "\0").unwrap();
    }
    #[test]
    fn all_different() {
        assert_eq!(::levenshtein("abc", "def"), Ok(3));
    }
    #[test]
    fn one_different() {
        assert_eq!(::levenshtein("abc", "aec"), Ok(1));
    }
    #[test]
    fn one_different2() {
        assert_eq!(::levenshtein("abcdefghijklmopqrstuvwxwz", "abcdefghijklmApqrstuvwxwz"), Ok(1));
    }
}

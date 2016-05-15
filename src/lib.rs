extern crate libc;


extern {
    fn levenshtein(a: *const libc::c_char, b: *const libc::c_char) -> libc::c_uint;
}

#[cfg(test)]
mod tests {
   use std::ffi::CString;
   #[test]
    fn it_works() {
        assert_eq!(unsafe{ ::levenshtein(CString::new("").unwrap().as_ptr(), CString::new("").unwrap().as_ptr()) }, 0);
    }
}

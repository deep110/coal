use std::ffi::CString;

pub fn cstring(value: &str) -> CString {
    CString::new(value).expect("some err")
}
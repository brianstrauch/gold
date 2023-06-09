use std::ffi::{c_char, CStr, CString};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub unsafe fn compile(expr: String) -> Option<String> {
    let expr = CString::new(expr).unwrap();
    let str = CStr::from_ptr(RegexpCompile(expr.as_ptr() as *mut c_char));
    let err = String::from_utf8_lossy(str.to_bytes()).to_string();
    GoFree(str.as_ptr() as *mut c_char);

    if err != "" {
        Some(err)
    } else {
        None
    }
}

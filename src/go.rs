use std::ffi::{c_char, CStr, CString};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn regexp_compile(expr: String) -> Option<String> {
    let expr = CString::new(expr).unwrap();

    unsafe {
        let str = CStr::from_ptr(RegexpCompile(expr.as_ptr() as *mut c_char));
        let err = String::from_utf8_lossy(str.to_bytes()).to_string();
        GoFree(str.as_ptr() as *mut c_char);
        if !err.is_empty() {
            return Some(err);
        }
    }

    None
}

pub fn html_template_new_parse(expr: String) -> Option<String> {
    let expr = CString::new(expr).unwrap();

    unsafe {
        let str = CStr::from_ptr(HtmlTemplateNewParse(expr.as_ptr() as *mut c_char));
        let err = String::from_utf8_lossy(str.to_bytes()).to_string();
        GoFree(str.as_ptr() as *mut c_char);
        if !err.is_empty() {
            return Some(err);
        }
    }

    None
}

pub fn text_template_new_parse(expr: String) -> Option<String> {
    let expr = CString::new(expr).unwrap();

    unsafe {
        let str = CStr::from_ptr(TextTemplateNewParse(expr.as_ptr() as *mut c_char));
        let err = String::from_utf8_lossy(str.to_bytes()).to_string();
        GoFree(str.as_ptr() as *mut c_char);
        if !err.is_empty() {
            return Some(err);
        }
    }

    None
}

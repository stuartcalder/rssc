use cty::*;

pub type ArgProc = unsafe extern "C" fn(
    wordc: c_int,
    wordv: *mut *mut c_char,
    offset: c_int,
    state: *mut c_void) -> c_int;

#[repr(C)]
pub struct ArgShort {
    proc: Option<ArgProc>,
    ch:   c_char,
}
impl ArgShort {
    fn new() -> ArgShort {
        ArgShort { proc: None, ch: 0 as c_char }
    }
}

#[repr(C)]
pub struct ArgLong {
    proc:      Option<ArgProc>,
    cstr:      Option<*const c_char>,
    cstr_size: size_t,
}
impl ArgLong {
    fn new() -> ArgLong {
        ArgLong { proc: None, cstr: None, cstr_size: 0usize }
    }
}

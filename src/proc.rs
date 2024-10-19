use std::ffi::{CString,CStr};
use libc;

#[link(name = "SSC")]
extern "C" {
    #[cfg(feature = "SSC_getExecutablePath")]
    fn SSC_getExecutablePath(
        store_path_size: *mut cty::size_t
    ) -> *mut cty::c_char;
    #[cfg(feature = "SSC_getNumberProcessors")]
    fn SSC_getNumberProcessors() -> cty::c_int;
}

#[cfg(feature = "SSC_getExecutablePath")]
pub fn get_executable_path() -> Result<CString, ()> {
    let mut size = 0usize;
    let path = unsafe {
        SSC_getExecutablePath(&mut size as *mut cty::size_t)
    };
    if path == std::ptr::null_mut() {
        return Err(())
    }
    let cstr    = unsafe { CStr::from_ptr(path) };
    let cstring = CString::from(cstr);
    unsafe { libc::free(path as *mut cty::c_void) };
    Ok(cstring)
}

#[cfg(feature = "SSC_getNumberProcessors")]
pub fn get_number_processors() -> cty::c_int {
    unsafe { SSC_getNumberProcessors() }
}

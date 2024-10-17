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
    if path != std::ptr::null_mut() {
        let path_cstr: &CStr = unsafe {
            CStr::from_ptr(path as *mut cty::c_char)
        };
        let cstr = CString::from(path_cstr);
        unsafe { libc::free(path as *mut cty::c_void) };
        Ok(cstr)
    } else {
        Err(())
    }
    //TODO
}

#[cfg(feature = "SSC_getNumberProcessors")]
pub fn get_number_processors() -> cty::c_int {
    unsafe { SSC_getNumberProcessors() }
}


//unsafe { std::slice::from_raw_parts_mut(self.get_ptr(), self.get_size()) }

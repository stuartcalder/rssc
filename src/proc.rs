/* *
 * rssc - Wrap the C library SSC in a Rust wrapper. (https://github.com/stuartcalder/SSC)
 * Copyright (C) 2025 Stuart Calder
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::ffi::{CString,CStr};
use libc;

#[link(name = "SSC")]
extern "C" {
    #[cfg(all(feature = "SSC_getExecutablePath", any(target_os = "linux", target_os = "windows")))]
    fn SSC_getExecutablePath(
        store_path_size: *mut cty::size_t
    ) -> *mut cty::c_char;
    #[cfg(all(feature = "SSC_getNumberProcessors", any(target_os = "linux", target_os = "windows")))]
    fn SSC_getNumberProcessors() -> cty::c_int;
}

#[cfg(all(feature = "SSC_getExecutablePath", any(target_os = "linux", target_os = "windows")))]
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

#[cfg(all(feature = "SSC_getNumberProcessors", any(target_os = "linux", target_os = "windows")))]
pub fn get_number_processors() -> cty::c_int {
    unsafe { SSC_getNumberProcessors() }
}

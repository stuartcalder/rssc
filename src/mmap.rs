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

use crate::c;
use std::ptr;
use cty::*;

pub mod file {
    use super::*;
    #[cfg(target_family = "unix")]
    pub type Type = c_int;
    #[cfg(target_family = "windows")]
    pub type Type = *mut c_void;

    pub const IS_INT: bool = cfg!(target_family = "unix");
    pub const IS_PTR: bool = cfg!(target_family = "windows");

    #[cfg(target_family = "unix")]
    pub const NULL: Type = -1;
    #[cfg(target_family = "windows")]
    pub const NULL: Type = -1isize as Type;
}

pub mod init_flag {
    use crate::c;
    pub const READ_ONLY:       c::BitFlag = 0x01; /* Disallow writing to memory-map. */
    pub const ALLOW_SHRINK:    c::BitFlag = 0x02; /* Allow shrinking the size of the mapped memory. */
    pub const FORCE_EXIST:     c::BitFlag = 0x04; /* Force a file to NOT exist, unless ForceExistYes is on... */
    pub const FORCE_EXIST_YES: c::BitFlag = 0x08; /* Force a file to exist, when @ForceExist is also on. */
}
pub mod init_code {
    use crate::c;
    pub const OK:                  c::CodeError =   0;
    pub const ERR_FILE_EXIST_NO:   c::CodeError =  -1; /* Failure to force non-existence of a file. */
    pub const ERR_FILE_EXIST_YES:  c::CodeError =  -2; /* Failure to force existence of a file. */
    pub const ERR_READONLY:        c::CodeError =  -3; /* Failure to enforce read-only. */
    pub const ERR_SHRINK:          c::CodeError =  -4; /* Attempted to shrink while disallowed */
    pub const ERR_NO_SIZE:         c::CodeError =  -5; /* Size not provided. */
    pub const ERR_OPEN_FILEPATH:   c::CodeError =  -6; /* Failed to open a filepath. */
    pub const ERR_CREATE_FILEPATH: c::CodeError =  -7; /* Failed to create a file at a filepath. */
    pub const ERR_GET_FILE_SIZE:   c::CodeError =  -8; /* Failed to get a file size. */
    pub const ERR_SET_FILE_SIZE:   c::CodeError =  -9; /* Failed to set a file size. */
    pub const ERR_MAP:             c::CodeError = -10; /* Failed to map a file into memory. */
}

#[repr(C)]
pub struct Map {
    ptr: *mut uint8_t,
    size: size_t,
    file: file::Type,
    #[cfg(target_family = "windows")]
    windows_filemap: file::Type,
    readonly: bool,
}

use std::ffi::CString;
impl Map {
    /// Return a default-initialized Memory Map.
    pub fn new_null() -> Self {
        Self {
            ptr:  ptr::null_mut::<uint8_t>(),
            size: 0usize,
            file: file::NULL,
            #[cfg(target_family = "windows")]
            windows_filemap: file::NULL,
            readonly: false,
        }
    }

    pub fn is_initialized(self: &Self) -> bool {
        ! self.ptr.is_null()
    }

    pub fn is_readonly(self: &Self) -> bool {
        self.readonly
    }

    pub fn sync(self: &mut Self) -> Result<(), c::Error> {
        let err = unsafe { SSC_MemMap_sync(self as *mut Self) };
        match err {
            0 => Ok(()),
            _ => Err(err),
        }
    }

    /// Initialize an existing Memory Map. Nullify if it's already initialized.
    pub fn init(
        self:     &mut Self,
        filepath: &CString,
        size:     size_t,
        flags:    c::BitFlag) -> Result<(), c::CodeError>
    {
        if self.is_initialized() {
            let err = self.nullify();
            match err {
                Err(_) => return Err(init_code::ERR_MAP), //TODO: Make me a more specific error? Why isn't C code handling this?
                _      => (),
            }
        }
        let code = unsafe {
            SSC_MemMap_init(
                self as *mut Self,
                filepath.as_ptr(),
                size,
                flags
            )
        };
        match code {
            init_code::OK => Ok(()),
            _             => Err(code)
        }
    }

    /// Return an initialized, mapped Memory Map.
    pub fn new(
        filepath: &CString,
        size:     size_t,
        flags:    c::BitFlag) -> Result<Self, c::CodeError>
    {
        let mut m = Self::new_null();
        m.init(filepath, size, flags)?;
        if flags & init_flag::READ_ONLY != 0 {
            m.readonly = true;
        }
        Ok(m)
    }

    /// Free Memory map's resources and nullify variables.
    pub fn nullify(&mut self) -> Result<(), c::Error> {
        if self.is_initialized() {
            if ! self.is_readonly() {
                let err = unsafe {
                    SSC_MemMap_sync(self as *const Self)
                };
                if err != 0 {
                    return Err(err);
                }
            }
            unsafe { SSC_MemMap_del(self as *mut Self) };
        }
        Ok(())
    }

    /// Return the (possibly) mapped memory as a mutable u8 pointer.
    pub fn get_ptr(&mut self) -> *mut uint8_t {
        self.ptr
    }

    /// Return the size of the (possibly) mapped file, or 0 if no file has been mapped.
    pub fn get_size(&self) -> size_t {
        self.size
    }

    /// Return whether we are allowed to write to the mapped memory of the Memory Map.
    pub fn get_readonly(&self) -> bool {
        self.readonly
    }

    /// Return a u8 slice representing the memory-mapped data.
    pub fn get_slice(&mut self) -> Option<&mut [uint8_t]> {
        if self.is_initialized() {
            Some(unsafe {std::slice::from_raw_parts_mut(self.get_ptr(), self.get_size())})
        } else {
            None
        }
    }

} // ~ impl Map
impl Drop for Map {
    /// When Dropped, check to see if the Map has been memory-mapped. If so call SSC_MemMap_del().
    fn drop(&mut self) {
        self.nullify().expect("Failed to Drop Map!");
    }
} // ~ impl Drop for Map

#[link(name = "SSC")]
extern "C" {
/* File procedures */
    fn SSC_FilePath_getSize(
        fpath:     *const c_char,
        storesize: *mut   size_t
    ) -> c::Error;
    fn SSC_FilePath_exists(
        fpath: *const c_char
    ) -> bool;
    fn SSC_FilePath_forceExistOrDie(
        fpath:   *const c_char,
        control: bool
    ) -> ();
    fn SSC_FilePath_open(
        fpath:     *const c_char,
        readonly:  bool, 
        storefile: *mut file::Type
    ) -> c::Error;
    fn SSC_FilePath_create(
        fpath:     *const c_char,
        storefile: *mut file::Type
    ) -> c::Error;
    fn SSC_File_getSize(
        file:      file::Type,
        storesize: *mut size_t
    ) -> c::Error;
    #[cfg(all(feature = "SSC_File_createSecret", target_os = "linux"))]
    fn SSC_File_createSecret(file: file::Type) -> c::Error;
    fn SSC_File_close(file: file::Type) -> c::Error;
    fn SSC_File_setSize(
        file: file::Type,
        size: size_t
    ) -> c::Error;
    fn SSC_chdir(fpath: *const c_char) -> c::Error;
/* MemMap procedures */
    fn SSC_MemMap_init(
        map:      *mut Map,
        filepath: *const c_char,
        size:     size_t,
        flags:    c::BitFlag
    ) -> c::CodeError;
    #[cfg(feature = "Disable")]
    fn SSC_MemMap_initOrDie(
        map:      *mut Map,
        filepath: *const c_char,
        size:     size_t,
        flags:    c::BitFlag
    ) -> ();
    fn SSC_MemMap_map(
        map: *mut Map,
        readonly: bool
    ) -> c::Error;
    fn SSC_MemMap_unmap(map: *mut Map)  -> ();
    fn SSC_MemMap_sync(map: *const Map) -> c::Error;
    fn SSC_MemMap_del(map: *mut Map)    -> ();
}

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

pub const HAS_INITSECRET: bool = cfg!(feature = "SSC_MemMap_initSecret") && cfg!(target_os = "linux");

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

    pub const HAS_CREATESECRET: bool = cfg!(feature = "SSC_File_createSecret") && cfg!(target_os = "linux");
}

pub mod flag {
    use crate::c::BitFlag8;
    pub const READ_ONLY:  BitFlag8 = 0x01u8;
    pub const SECRET:     BitFlag8 = 0x02u8;
}
pub mod init_flag {
    use crate::c::BitFlag;
    pub const READ_ONLY:       BitFlag = 0x01; /* Disallow writing to memory-map. */
    pub const ALLOW_SHRINK:    BitFlag = 0x02; /* Allow shrinking the size of the mapped memory. */
    pub const FORCE_EXIST:     BitFlag = 0x04; /* Force a file to NOT exist, unless ForceExistYes is on... */
    pub const FORCE_EXIST_YES: BitFlag = 0x08; /* Force a file to exist, when @ForceExist is also on. */
}
pub mod init_code {
    use crate::c::CodeError;
    pub const OK:                   CodeError =   0;
    pub const ERR_FILE_EXIST_NO:    CodeError =  -1; /* Failure to force non-existence of a file. */
    pub const ERR_FILE_EXIST_YES:   CodeError =  -2; /* Failure to force existence of a file. */
    pub const ERR_READ_ONLY:        CodeError =  -3; /* Failure to enforce read-only. */
    pub const ERR_SHRINK:           CodeError =  -4; /* Attempted to shrink while disallowed */
    pub const ERR_NO_SIZE:          CodeError =  -5; /* Size not provided. */
    pub const ERR_OPEN_FILEPATH:    CodeError =  -6; /* Failed to open a filepath. */
    pub const ERR_CREATE_FILEPATH:  CodeError =  -7; /* Failed to create a file at a filepath. */
    pub const ERR_GET_FILE_SIZE:    CodeError =  -8; /* Failed to get a file size. */
    pub const ERR_SET_FILE_SIZE:    CodeError =  -9; /* Failed to set a file size. */
    pub const ERR_MAP:              CodeError = -10; /* Failed to map a file into memory. */
    pub const ERR_SECRET:           CodeError = -11; /* Failed to initialize a secret map. */
    /* Error codes underneath this comment are Rust-specific, and never emitted by the C code. */
    pub const ERR_NULLIFY:          CodeError = -12; /* Failed to nullify prior to initialization. */
}

#[repr(C)]
pub struct Map {
    ptr: *mut uint8_t,
    size: size_t,
    file: file::Type,
    #[cfg(target_family = "windows")]
    windows_filemap: file::Type,
    flags: c::BitFlag8,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            ptr:  ptr::null_mut::<uint8_t>(),
            size: 0usize,
            file: file::NULL,
            #[cfg(target_family = "windows")]
            windows_filemap: file::NULL,
            flags: 0u8,
        }
    }
}

impl Drop for Map {
    /// When Dropped, check to see if the Map has been memory-mapped. If so call SSC_MemMap_del().
    fn drop(&mut self) {
        self.nullify().expect("Failed to Drop Map!");
    }
}

use std::ffi::CString;
impl Map {
    /// Return a default-initialized Memory Map.
    pub fn new_null() -> Self {
        Map::default()
    }

    /// Is the Memory Map already mapped?
    pub fn is_initialized(&self) -> bool {
        ! self.ptr.is_null()
    }

    /// Is the Memory Map marked read-only? i.e. no write operations.
    pub fn is_readonly(&self) -> bool {
        (self.flags & flag::READ_ONLY) != 0u8
    }

    /// Does this implementation support creating Secret Maps?
    pub fn supports_secret() -> bool {
        if !HAS_INITSECRET {
            return false;
        }
        unsafe { SSC_File_createSecretIsAvailable() }
    }

    /// Is this Memory Map a Secret Map?
    pub fn is_secret(&self) -> bool {
        (self.flags & flag::SECRET) != 0u8
    }

    /// Synchronize the Memory Map's data with the filesystem.
    pub fn sync(&mut self) -> Result<(), ()> {
        let err = unsafe { SSC_MemMap_sync(self as *mut Self) };
        match err {
            0 => Ok(()),
            _ => Err(()),
        }
    }

    /// Initialize an existing Memory Map. Nullify if it's already initialized.
    pub fn init(
        &mut self,
        filepath: &CString,
        size:     size_t,
        flags:    c::BitFlag) -> Result<(), c::CodeError>
    {
        if self.is_initialized() {
            if self.nullify().is_err() {
                return Err(init_code::ERR_NULLIFY);
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

    ///TODO
    #[cfg(all(feature = "SSC_MemMap_initSecret", target_os = "linux"))]
    pub fn init_secret(
        &mut self,
        size: size_t) -> Result<(), c::CodeError>
    {
        if self.is_initialized() {
            if self.nullify().is_err() {
                return Err(init_code::ERR_NULLIFY);
            }
        }
        let code = unsafe {
            SSC_MemMap_initSecret(
                self as *mut Self,
                size
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
            m.flags |= flag::READ_ONLY;
        }
        Ok(m)
    }

    ///TODO
    #[cfg(all(feature = "SSC_MemMap_initSecret", target_os = "linux"))]
    pub fn new_secret(size: size_t) -> Result<Self, c::CodeError>
    {
        let mut m = Self::new_null();
        m.init_secret(size)?;
        Ok(m)
    }

    /// Free Memory map's resources and nullify variables.
    pub fn nullify(&mut self) -> Result<(), ()> {
        if self.is_initialized() {
            if ! self.is_readonly() {
                let err = unsafe {
                    SSC_MemMap_sync(self as *const Self)
                };
                if err != 0 {
                    return Err(());
                }
            }
            unsafe { SSC_MemMap_del(self as *mut Self) };
        }
        *self = Self::default();
        Ok(())
    }

    /// Increase or decrease the size of the Memory Map.
    pub fn resize(&mut self, size: size_t) -> Result<(), ()> {
        let err = unsafe { SSC_MemMap_resize(self as *mut Self, size) };
        match err {
            0 => Ok(()),
            _ => Err(()),
        }
    }

    /// Return the (possibly) mapped memory as a mutable u8 pointer.
    pub fn get_ptr(&mut self) -> *mut uint8_t {
        self.ptr
    }

    /// Return the size of the (possibly) mapped file, or 0 if no file has been mapped.
    pub fn get_size(&self) -> size_t {
        self.size
    }

    /// Return the 8 bits that serve as the flags.
    pub fn get_flags(&self) -> c::BitFlag8 {
        self.flags
    }

    /// Return a reference to a u8 slice representing the memory-mapped data.
    pub fn get_slice(&mut self) -> Option<&mut [uint8_t]> {
        if self.is_initialized() {
            Some(unsafe {std::slice::from_raw_parts_mut(self.get_ptr(), self.get_size())})
        } else {
            None
        }
    }

} // ~ impl Map

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
    fn SSC_File_createSecretIsAvailable() -> bool;
    fn SSC_File_close(file: file::Type) -> c::Error;
    fn SSC_File_setSize(
        file: file::Type,
        size: size_t
    ) -> c::Error;
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
    #[cfg(all(feature = "SSC_MemMap_initSecret", target_os = "linux"))]
    fn SSC_MemMap_initSecret(map: *mut Map, size: size_t) -> c::CodeError;
    fn SSC_MemMap_resize(map: *mut Map, size: size_t) -> c::Error;
/* Misc procedures */
    fn SSC_chdir(fpath: *const c_char) -> c::Error;
}

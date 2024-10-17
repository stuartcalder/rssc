

use crate::c;

pub mod file {
    #[cfg(target_family = "unix")]
    pub type Type = cty::c_int;
    #[cfg(target_family = "windows")]
    pub type Type = *mut cty::c_void;

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
    ptr: *mut cty::uint8_t,
    size: cty::size_t,
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
            ptr: std::ptr::null_mut::<cty::uint8_t>(),
            size: 0usize,
            file: file::NULL,
            #[cfg(target_family = "windows")]
            windows_filemap: file::NULL,
            readonly: false,
        }
    }

    pub fn is_initialized(self: &Self) -> bool {
        self.ptr != std::ptr::null_mut()
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
        size:     cty::size_t,
        flags:    c::BitFlag
    ) -> Result<(), c::CodeError>
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
        size:     cty::size_t,
        flags:    c::BitFlag
    ) -> Result<Self, c::CodeError>
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
    pub fn get_ptr(&mut self) -> *mut cty::uint8_t {
        self.ptr
    }

    /// Return the size of the (possibly) mapped file, or 0 if no file has been mapped.
    pub fn get_size(&self) -> cty::size_t {
        self.size
    }

    /// Return whether we are allowed to write to the mapped memory of the Memory Map.
    pub fn get_readonly(&self) -> bool {
        self.readonly
    }

    /// Return a u8 slice representing the memory-mapped data.
    pub fn get_slice(&mut self) -> &mut [cty::uint8_t] {
        if self.is_initialized() {
            unsafe { std::slice::from_raw_parts_mut(self.get_ptr(), self.get_size()) }
        } else {
            &mut []
        }
    }

} // ~ impl Map
impl Drop for Map {
    /// When Dropped, check to see if the Map has been memory-mapped. If so call SSC_MemMap_del().
    fn drop(&mut self) {
        self.nullify().unwrap();
    }
} // ~ impl Drop for Map

#[link(name = "SSC")]
extern "C" {
/* File procedures */
    pub fn SSC_FilePath_getSize(
        fpath: *const cty::c_char,
        storesize: *mut cty::size_t
    ) -> c::Error;
    pub fn SSC_FilePath_exists(
        fpath: *const cty::c_char
    ) -> bool;
    pub fn SSC_FilePath_forceExistOrDie(
        fpath: *const cty::c_char,
        control: bool
    ) -> ();
    pub fn SSC_FilePath_open(
        fpath: *const cty::c_char,
        readonly: bool, 
        storefile: *mut file::Type
    ) -> c::Error;
    pub fn SSC_FilePath_create(
        fpath: *const cty::c_char,
        storefile: *mut file::Type
    ) -> c::Error;
    pub fn SSC_File_getSize(
        file: file::Type,
        storesize: *mut cty::size_t
    ) -> c::Error;
    #[cfg(all(feature = "SSC_File_createSecret", target_os = "linux"))]
    pub fn SSC_File_createSecret(file: file::Type) -> c::Error;
    pub fn SSC_File_close(file: file::Type) -> c::Error;
    pub fn SSC_File_setSize(
        file: file::Type,
        size: cty::size_t
    ) -> c::Error;
    pub fn SSC_chdir(fpath: *const cty::c_char) -> c::Error;
/* MemMap procedures */
    pub fn SSC_MemMap_init(
        map:      *mut Map,
        filepath: *const cty::c_char,
        size:     cty::size_t,
        flags:    c::BitFlag
    ) -> c::CodeError;
    #[cfg(feature = "Disable")]
    pub fn SSC_MemMap_initOrDie(
        map:      *mut Map,
        filepath: *const cty::c_char,
        size:     cty::size_t,
        flags:    c::BitFlag
    ) -> ();
    pub fn SSC_MemMap_map(
        map: *mut Map,
        readonly: bool
    ) -> c::Error;
    pub fn SSC_MemMap_unmap(map: *mut Map)  -> ();
    pub fn SSC_MemMap_sync(map: *const Map) -> c::Error;
    pub fn SSC_MemMap_del(map: *mut Map)    -> ();
}

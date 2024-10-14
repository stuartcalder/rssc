
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

pub mod mmap {
    enum Init {
        ReadOnly      = 0x01,
        AllowShrink   = 0x02,
        ForceExist    = 0x04,
        ForceExistYes = 0x08,
    }
    enum InitCode {
        Okay              =  0,
        ErrFileExistNo    = -1, /* Failure to force non-existence of a file. */
        ErrFileExistYes   = -2, /* Failure to force existence of a file. */
        ErrReadOnly       = -3, /* Failure to enforce read-only. */
        ErrShrink         = -4, /* Attempted to shrink while disallowed */
        ErrNoSize         = -5, /* Size not provided. */
        ErrOpenFilePath   = -6, /* Failed to open a filepath. */
        ErrCreateFilePath = -7, /* Failed to create a file at a filepath. */
        ErrGetFileSize    = -8, /* Failed to get a file size. */
        ErrSetFileSize    = -9, /* Failed to set a file size. */
        ErrMap            = -10, /* Failed to map a file into memory. */
    }

    #[repr(C)]
    pub struct Map {
        ptr: *mut cty::uint8_t,
        size: cty::size_t,
        file: super::file::Type,
        #[cfg(target_family = "windows")]
        windows_filemap: super::file::Type,
        readonly: bool,
    }

    impl Map {
        pub fn new() -> Map {
            Map {
                ptr: std::ptr::null_mut::<cty::uint8_t>(),
                size: 0usize,
                file: super::file::NULL,
                #[cfg(target_family = "windows")]
                windows_filemap: super::file::NULL,
                readonly: false,
            }
        }
        pub fn nullify(&mut self) {
            if self.ptr != std::ptr::null_mut() {
                unsafe { super::SSC_MemMap_del(self) };
            }
            self.ptr = std::ptr::null_mut::<cty::uint8_t>();
            self.size = 0usize;
            self.file = super::file::NULL;
            #[cfg(target_family = "windows")]
            {
                self.windows_filemap = super::file::NULL;
            }
            self.readonly = false;
        }
        pub fn as_ptr(&mut self) -> *mut Map {
            self as *mut Map
        }
    } // ~ impl Map
    impl Drop for Map {
        fn drop(&mut self) {
            if self.ptr != std::ptr::null_mut() {
                unsafe { super::SSC_MemMap_del(self) };
            }
        }
    } // ~ impl Drop for Map
}

#[link(name = "SSC")]
extern "C" {
/* File procedures */
    pub fn SSC_File_getSize(file: file::Type, storesize: *mut cty::size_t) -> c::Error;
    pub fn SSC_FilePath_getSize(fpath: *const cty::c_char, storesize: *mut cty::size_t) -> c::Error;
    pub fn SSC_FilePath_exists(fpath: *const cty::c_char) -> bool;
    pub fn SSC_FilePath_forceExistOrDie(fpath: *const cty::c_char, control: bool) -> ();
    pub fn SSC_FilePath_open(
        fpath: *const cty::c_char,
        readonly: bool, 
        storefile: *mut file::Type) -> c::Error;
    pub fn SSC_FilePath_create(
        fpath: *const cty::c_char,
        storefile: *mut file::Type) -> c::Error;
    #[cfg(all(feature = "SSC_File_createSecret", target_os = "linux"))]
    pub fn SSC_File_createSecret(file: file::Type) -> c::Error;
    pub fn SSC_File_close(file: file::Type) -> c::Error;
    pub fn SSC_File_setSize(file: file::Type, size: cty::size_t) -> c::Error;
    pub fn SSC_chdir(fpath: *const cty::c_char) -> c::Error;
/* MemMap procedures */
    pub fn SSC_MemMap_init(
        map:      *mut mmap::Map,
        filepath: *const cty::c_char,
        size:     cty::size_t,
        flags:    c::BitFlag) -> c::CodeError;
    pub fn SSC_MemMap_initOrDie(
        map:      *mut mmap::Map,
        filepath: *const cty::c_char,
        size:     cty::size_t,
        flags:    c::BitFlag) -> ();
    pub fn SSC_MemMap_map(
        map: *mut mmap::Map,
        readonly: bool) -> c::Error;
    pub fn SSC_MemMap_unmap(map: *mut mmap::Map) -> ();
    pub fn SSC_MemMap_sync(map: *const mmap::Map) -> c::Error;
    pub fn SSC_MemMap_del(map: *mut mmap::Map) -> ();
}


pub type ErrorT = cty::c_int;

#[cfg(target_family = "unix")]
pub mod file {
    pub type CType = cty::c_int;
    pub const IS_INT: bool = true;
    pub const IS_PTR: bool = false;
    pub const NULL:  CType = -1;
}

#[cfg(target_family = "windows")]
pub mod file {
    pub type CType = *mut cty::c_void;
    //use *mut cty::c_void as File_t;
    pub const IS_INT: bool = false;
    pub const IS_PTR: bool = true;
    pub const NULL:  CType = -1 as c_type;
}

#[link(name = "SSC")]
extern "C" {
    pub fn SSC_File_getSize(file: file::CType, storesize: *mut cty::size_t) -> ErrorT;
    pub fn SSC_FilePath_getSize(fpath: *const cty::c_char, storesize: *mut cty::size_t) -> ErrorT;
    pub fn SSC_FilePath_exists(fpath: *const cty::c_char) -> bool;
    pub fn SSC_FilePath_forceExistOrDie(fpath: *const cty::c_char, control: bool) -> ();
    pub fn SSC_FilePath_open(
        fpath: *const cty::c_char,
        readonly: bool, 
        storefile: *mut file::CType) -> ErrorT;
    pub fn SSC_FilePath_create(
        fpath: *const cty::c_char,
        storefile: *mut file::CType) -> ErrorT;
    /* TODO: Implement SSC_File_createSecret() later? */
    pub fn SSC_File_close(file: file::CType) -> ErrorT;
    pub fn SSC_File_setSize(file: file::CType, size: cty::size_t) -> ErrorT;
    pub fn SSC_chdir(fpath: *const cty::c_char) -> ErrorT;
}

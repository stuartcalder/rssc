
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

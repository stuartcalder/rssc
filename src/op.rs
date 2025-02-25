pub use cty::{c_void, size_t};

#[link(name = "SSC")]
extern "C" {
/* Memory procedures. */
    pub fn SSC_constTimeMemDiff(
        mem_0: *const c_void,
        mem_1: *const c_void,
        size:  size_t
    ) -> size_t;
    pub fn SSC_isZero(
        mem: *const c_void,
        size: size_t
    ) -> bool;
    pub fn SSC_constTimeIsZero(
        mem: *const c_void,
        size: size_t
    ) -> bool;
    pub fn SSC_secureZero(
        mem: *mut c_void,
        size: size_t
    ) -> ();
}

pub fn secure_zero(bytes: &mut [u8]) {
    unsafe {
        SSC_secureZero(
            bytes as *mut _ as *mut c_void,
            bytes.len()
        )
    }
}

#[allow(unused)]
macro_rules! secure_drop {
    () => {unsafe {
        SSC_secureZero(
            self as *mut _ as *mut c_void,
            std::mem::size_of::<Self>()
        )
    }}
}

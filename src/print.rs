
#[link(name = "SSC")]
extern "C" {
    fn SSC_printBytes(
        mem:  *const cty::c_void,
        size: cty::size_t
    ) -> ();
}

pub fn print_bytes(bytes: &[u8]) -> () {
    unsafe {
        SSC_printBytes(
            bytes.as_ptr() as *const _ as *const cty::c_void,
            bytes.len()
        )
    }
}


#[link(name = "SSC")]
extern "C" {
    fn SSC_getEntropy(
        mem: *mut cty::c_void,
        size: cty::size_t
    ) -> ();
}

pub fn get_entropy(bytes: &mut [u8]) -> () {
    unsafe {
        SSC_getEntropy(
            bytes as *mut _ as *mut cty::c_void,
            bytes.len()
        )
    }
}

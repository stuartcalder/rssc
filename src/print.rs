
#[link(name = "SSC")]
extern "C" {
    pub fn SSC_printBytes(
        mem:  *const cty::c_void,
        size: cty::size_t
    ) -> ();
}

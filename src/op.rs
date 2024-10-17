
#[link(name = "SSC")]
extern "C" {
/* Memory procedures. */
    pub fn SSC_constTimeMemDiff(
        mem_0: *const cty::c_void,
        mem_1: *const cty::c_void,
        size:  cty::size_t
    ) -> cty::size_t;
    pub fn SSC_isZero(
        mem: *const cty::c_void,
        size: cty::size_t
    ) -> bool;
    pub fn SSC_constTimeIsZero(
        mem: *const cty::c_void,
        size: cty::size_t
    ) -> bool;
}

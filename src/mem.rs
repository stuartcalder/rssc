
#[link(name = "SSC")]
extern "C" {
    #[cfg(feature = "SSC_getTotalSystemMemory")]
    fn SSC_getTotalSystemMemory() -> cty::size_t;
    #[cfg(feature = "SSC_getAvailableSystemMemory")]
    fn SSC_getAvailableSystemMemory() -> cty::size_t;
}

#[cfg(feature = "SSC_getTotalSystemMemory")]
pub fn get_total_system_memory() -> usize {
    unsafe { SSC_getTotalSystemMemory() }
}

#[cfg(feature = "SSC_getAvailableSystemMemory")]
pub fn get_available_system_memory() -> usize {
    unsafe { SSC_getAvailableSystemMemory() }
}

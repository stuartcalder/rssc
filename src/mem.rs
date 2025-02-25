
#[link(name = "SSC")]
extern "C" {
    #[cfg(all(feature = "SSC_getTotalSystemMemory", any(target_os = "linux", target_os = "windows")))]
    fn SSC_getTotalSystemMemory() -> cty::size_t;
    #[cfg(all(feature = "SSC_getAvailableSystemMemory", any(target_family = "unix", target_os = "windows")))]
    fn SSC_getAvailableSystemMemory() -> cty::size_t;
}

pub struct Memory {
    value: usize
}
pub const KI: usize = 1024usize;
pub const MI: usize = KI * KI;
pub const GI: usize = MI * KI;
pub const TI: usize = GI * KI;

impl Memory {
    pub fn get(&self) -> usize {
        self.value
    }
    pub fn get_as<const UNIT: usize>(&self) -> usize {
        self.get() / UNIT
    }
}
impl Copy for Memory { }
impl Clone for Memory {
    fn clone(&self) -> Memory {
        *self
    }
}

#[cfg(all(feature = "SSC_getTotalSystemMemory", any(target_os = "linux", target_os = "windows")))]
pub fn get_total_system_memory() -> Memory {
    let v = unsafe { SSC_getTotalSystemMemory()};
    Memory { value: v }
}

#[cfg(all(feature = "SSC_getAvailableSystemMemory", any(target_family = "unix", target_os = "windows")))]
pub fn get_available_system_memory() -> Memory {
    let v = unsafe { SSC_getAvailableSystemMemory() };
    Memory { value: v }
}

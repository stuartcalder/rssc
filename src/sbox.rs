pub use cty::c_void;
use crate::op;

#[repr(transparent)]
struct SBoxSlice<T>(Box::<[T]>);

impl<T> Drop for SBoxSlice::<T> {
    fn drop(&mut self) {
        let ptr  = &mut self.0 as *mut _ as *mut c_void;
        let size = self.0.len();
        unsafe {
            op::SSC_secureZero(ptr, size);
        }
    }
}

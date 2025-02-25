/* *
 * rssc - Wrap the C library SSC in a Rust wrapper. (https://github.com/stuartcalder/SSC)
 * Copyright (C) 2025 Stuart Calder
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

pub use cty::c_void;
use crate::op;

#[repr(transparent)]
struct SBox<T>(pub Box::<T>);

#[repr(transparent)]
struct SBoxSlice<T>(pub Box::<[T]>);

impl<T> Drop for SBox::<T> {
    fn drop(&mut self) {
        let ptr = &mut self.0 as *mut _ as *mut c_void;
        let size = std::mem::size_of::<T>();
        unsafe {
            op::SSC_secureZero(ptr, size);
        }
    }
}

impl<T> Drop for SBoxSlice::<T> {
    fn drop(&mut self) {
        let ptr  = &mut self.0 as *mut _ as *mut c_void;
        let size = self.0.len();
        unsafe {
            op::SSC_secureZero(ptr, size);
        }
    }
}

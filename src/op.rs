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

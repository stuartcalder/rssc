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

pub trait Integer {}

impl Integer for i8    {}
impl Integer for i16   {}
impl Integer for i32   {}
impl Integer for i64   {}
impl Integer for i128  {}
impl Integer for isize {}

impl Integer for u8    {}
impl Integer for u16   {}
impl Integer for u32   {}
impl Integer for u64   {}
impl Integer for u128  {}
impl Integer for usize {}

pub trait Unsigned {}

impl Unsigned for u8    {}
impl Unsigned for u16   {}
impl Unsigned for u32   {}
impl Unsigned for u64   {}
impl Unsigned for u128  {}
impl Unsigned for usize {}

/// Securely zero over all the bytes underlying the slice @bytes.
pub fn secure_zero<T>(bytes: &mut [T])
where T: Integer
{
    unsafe {
        SSC_secureZero(
            bytes as *mut _ as *mut c_void,
            bytes.len() * std::mem::size_of::<T>()
        )
    }
}

/// Return the number of bytes differing between @m0 and @m1.
pub fn const_time_mem_diff<T>(
    m0:   &[T],
    m1:   &[T]) -> Result<size_t, ()>
where T: Unsigned
{
    if m0.len() != m1.len() {
        return Err(());
    }
    let s = unsafe {
        SSC_constTimeMemDiff(
            m0 as *const _ as *const c_void,
            m1 as *const _ as *const c_void,
            m0.len() * std::mem::size_of::<T>()
        )
    };
    Ok(s)
}

/// Are the bytes of @mem all zero? True/False.
pub fn const_time_is_zero<T>(mem: &[T]) -> bool
where T: Unsigned
{
    unsafe {
        SSC_constTimeIsZero(
            mem as *const _ as *const c_void,
            mem.len() * std::mem::size_of::<T>()
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

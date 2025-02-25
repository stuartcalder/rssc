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
use cty::*;

pub type ArgProc = unsafe extern "C" fn(
    wordc: c_int,
    wordv: *mut *mut c_char,
    offset: c_int,
    state: *mut c_void) -> c_int;

#[repr(C)]
pub struct ArgShort {
    proc: Option<ArgProc>,
    ch:   c_char,
}
impl ArgShort {
    fn new() -> ArgShort {
        ArgShort { proc: None, ch: 0 as c_char }
    }
}

#[repr(C)]
pub struct ArgLong {
    proc:      Option<ArgProc>,
    cstr:      Option<*const c_char>,
    cstr_size: size_t,
}
impl ArgLong {
    fn new() -> ArgLong {
        ArgLong { proc: None, cstr: None, cstr_size: 0usize }
    }
}

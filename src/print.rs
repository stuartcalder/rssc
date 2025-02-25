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


#[link(name = "SSC")]
extern "C" {
    fn SSC_printBytes(
        mem:  *const cty::c_void,
        size: cty::size_t
    ) -> ();
}

pub fn print_bytes(bytes: &[u8]) -> () {
    unsafe {
        SSC_printBytes(
            bytes.as_ptr() as *const _ as *const cty::c_void,
            bytes.len()
        )
    }
}

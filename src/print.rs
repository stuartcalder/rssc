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

use crate::c::BitFlag8;

#[link(name = "SSC")]
extern "C" {
    fn SSC_printBytesMode(
        mem:  *const cty::c_void,
        size: cty::size_t,
        mode: BitFlag8
    ) -> ();
}

pub const MODE_HEX: u8     = 0x01u8; // Print bytes in hexadecimal.
pub const MODE_BIN: u8     = 0x02u8; // Print bytes in binary.
pub const MODE_PREFIX: u8  = 0x04u8; // Print the "prefix". i.e. 0x or 0b.
pub const MODE_NEWLINE: u8 = 0x08u8; // Print a newline after the formatted output.

pub fn print_bytes(bytes: &[u8], mode: BitFlag8) -> () {
    unsafe {
        SSC_printBytesMode(
            bytes.as_ptr() as *const _ as *const cty::c_void,
            bytes.len(),
            mode
        )
    }
}

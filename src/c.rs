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

use cty::{c_uint,uint8_t,uint16_t,uint32_t,uint64_t};
use cty::{c_int,int32_t,int64_t};
pub type Error      = cty::c_int;

pub type BitError   = cty::c_uint;
pub type BitError8  = cty::uint8_t;
pub type BitError16 = cty::uint16_t;
pub type BitError32 = cty::uint32_t;
pub type BitError64 = cty::uint64_t;

pub type CodeError   = cty::c_int;
pub type CodeError32 = cty::int32_t;
pub type CodeError64 = cty::int64_t;

pub type BitFlag   = cty::c_uint;
pub type BitFlag8  = cty::uint8_t;
pub type BitFlag16 = cty::uint16_t;
pub type BitFlag32 = cty::uint32_t;
pub type BitFlag64 = cty::uint64_t;

#[link(name = "SSC")]
extern "C" {
/* Process procedures */
    #[cfg(feature = "SSC_getNumberProcessors")]
    pub fn SSC_getNumberProcessors() -> cty::c_int;
}

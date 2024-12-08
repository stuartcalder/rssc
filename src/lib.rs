#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))] // Warn about this stuff on Release mode only.
use std::ptr;
//pub mod c;
pub mod c;
pub mod mmap;
pub mod op;
pub mod print;
pub mod rand;
pub mod proc;
pub mod mem;
pub mod cli;
pub mod sbox;

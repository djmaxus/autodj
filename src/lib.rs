#![doc = include_str!("../README.md")]
#![no_std]

extern crate no_std_compat as std;

pub mod prelude; // NOTE: do not use inside the library itself

pub mod fluid;
pub mod solid;

#[cfg(test)]
mod tests;

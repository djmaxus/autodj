#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod prelude; // NOTE: do not use inside the library itself

pub mod fluid;
pub mod solid;

#[cfg(test)]
mod tests;

#![warn(missing_docs)]
//! # Automatic Differentiation Library
//!
//! by [djmaxus](https://djmaxus.github.io/)
//!
//! ## References
//!
//! - [Wikipedia](https://en.wikipedia.org/wiki/Dual_number)
//! - [Automatic Differentiation](https://en.wikipedia.org/wiki/Automatic_differentiation)

pub mod single;

pub mod array;

pub mod vector;

pub mod common;

#[cfg(test)]
mod tests;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

// NOTE: test coverage setup
// - https://doc.rust-lang.org/rustc/instrument-coverage.html
// - https://blog.rng0.io/how-to-do-code-coverage-in-rust

#![warn(
    missing_docs,
    clippy::pedantic,
    clippy::type_repetition_in_bounds,
    unused,
    unreachable_pub,
    unused_results,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications
)]
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
mod tests {}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

// NOTE: test coverage setup
// - https://doc.rust-lang.org/rustc/instrument-coverage.html
// - https://blog.rng0.io/how-to-do-code-coverage-in-rust
// - https://about.codecov.io/
// - https://coveralls.io/

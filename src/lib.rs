#![warn(
    missing_docs,
    unreachable_pub,
    private_in_public,
    pub_use_of_private_extern_crate,
    exported_private_dependencies,
    unused,
    unused_results,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    rustdoc::private_doc_tests,
    rustdoc::private_intra_doc_links,
    rustdoc::all,
    rustdoc::missing_crate_level_docs,
    clippy::pedantic,
    clippy::suspicious,
    clippy::perf,
    clippy::style,
    clippy::complexity,
    clippy::type_repetition_in_bounds,
    clippy::allow_attributes_without_reason,
    clippy::as_conversions,
    clippy::clone_on_ref_ptr,
    clippy::dbg_macro,
    clippy::default_numeric_fallback,
    clippy::exhaustive_enums,
    clippy::expect_used,
    clippy::float_cmp_const,
    clippy::if_then_some_else_none,
    clippy::impl_trait_in_params,
    clippy::lossy_float_literal,
    clippy::missing_docs_in_private_items,
    clippy::redundant_pub_crate,
    clippy::cargo,
    clippy::disallowed_methods
)]
#![doc = include_str!("../README.md")]

pub mod fluid;
pub mod solid;
pub use solid::{array, single, sparse, vector};

#[cfg(test)]
mod tests;

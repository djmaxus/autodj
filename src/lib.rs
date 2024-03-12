#![warn(
    missing_docs,
    unreachable_pub,
    private_interfaces,
    private_bounds,
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
    clippy::disallowed_methods,
    clippy::unwrap_used,
    clippy::fallible_impl_from,
    clippy::get_unwrap,
    clippy::map_unwrap_or,
    clippy::unnecessary_safety_comment,
    clippy::unwrap_in_result,
    clippy::cast_lossless,
    clippy::indexing_slicing,
    clippy::trivially_copy_pass_by_ref
)]
#![doc = include_str!("../README.md")]

pub mod fluid;
pub mod solid;

#[cfg(test)]
mod tests;

// NOTE: do not use inside the library itself
/// Set of preludes
pub mod prelude {
    /// Prelude for working with the trait [`crate::fluid::Dual`]
    pub mod fluid {
        pub use crate::fluid::{Dual, Grad, Value};
    }

    /// Prelude for working with generic [`crate::solid::DualNumber`]
    pub mod solid {
        pub use crate::fluid::{Dual, Grad, Value};
        pub use crate::solid::DualNumber;
    }

    /// Prelude for working with [`crate::solid::single::DualNumber`]
    pub mod single {
        pub use crate::fluid::{Dual, Grad, Value};

        pub use crate::solid::single::*;
    }

    /// Prelude for working with [`crate::solid::array::DualNumber`]
    pub mod array {
        pub use crate::fluid::{Dual, Grad, Value};

        pub use crate::solid::array::*;
    }

    /// Prelude for working with [`crate::solid::vector::DualNumber`]
    pub mod vector {
        pub use crate::fluid::{Dual, Grad, Value};

        pub use crate::solid::vector::*;
    }

    /// Prelude for working with [`crate::solid::sparse::uuid::DualNumber`]
    pub mod uuid {
        pub use crate::fluid::{Dual, Grad, Value};

        pub use crate::solid::sparse::{self, uuid::*};
    }
}

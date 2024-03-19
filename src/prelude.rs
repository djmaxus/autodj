//!Set of preludes

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
#[cfg(feature = "uuid")]
pub mod uuid {
    pub use crate::fluid::{Dual, Grad, Value};

    pub use crate::solid::sparse::{self, uuid::*};
}

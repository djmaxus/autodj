//! [`crate::solid::DualNumber`] for single variable differentiations

pub use crate::solid::*;

/// Uni-variate dual number
///
///```
/// use autodj::single::*;
/// let x0 : DualF64 = 1.0.into();
/// let x  : DualF64 = 3.0.into_variable();
/// let f = (x - x0).powf(2.0);
/// assert_eq!(f.value(), &4.);
/// assert_eq!(f.dual(),  &4.);
/// ```
pub type DualNumber<V> = super::DualNumber<V, V>;

/// Single [`f64`] variable
pub type DualF64 = DualNumber<f64>;

/// Single [`f32`] variable
pub type DualF32 = DualNumber<f32>;

// TODO: is it generalizable for multivariate ?
/// Create an independent variable from a value
pub trait IntoVariable: Value {
    /// Create an independent variable from a value
    fn into_variable(self) -> DualNumber<Self> {
        Dual::new(self, Self::one())
    }
}

impl<V: Value> IntoVariable for V {}

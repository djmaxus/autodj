//! [`crate::single::DualNumber`] for single variable differentiations

/// Single variable specialization
///```
/// # use autodj::single::*;
/// let x0 : DualNumber = 1.0.into(); // Parameter
/// let x = 3.0.into_variable();
/// let f = (x - x0).powi(2);
/// assert_eq!(f.value(), 4.);
/// assert_eq!(f.deriv(), 4.);
/// ```
pub type DualNumber = DualCommon<f64>;

impl Copy for DualNumber {}

impl DualNumber {
    /// Construct a variable
    pub fn variable(real: f64) -> Self {
        Self { real, dual: 1. }
    }
    /// Get the derivative (dual component)
    pub fn deriv(&self) -> f64 {
        self.dual
    }
    /// Evaluate a function over the dual number
    pub fn eval<Out>(&self, func: impl Fn(DualNumber) -> Out) -> Out {
        func(*self)
    }
}

/// Trait for a type that can be converted into a single variable
pub trait IntoVariable {
    /// Convert into a variable
    fn into_variable(self) -> DualNumber;
}

impl IntoVariable for f64 {
    fn into_variable(self) -> DualNumber {
        DualNumber::variable(self)
    }
}

impl DualComponent for f64 {
    fn zero() -> Self {
        0.
    }
}

use crate::common::{DualCommon, DualComponent};

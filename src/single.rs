//! [`crate::single::DualNumber`] for single variable differentiations

use crate::common::Common;
pub use crate::fluid::Dual;

/// Single variable specialization
///```
/// # use autodj::single::*;
/// let x0 : DualNumber = 1.0.into(); // Parameter
/// let x = 3.0.into_variable();
/// let f = (x - x0).powf(2.0);
/// assert_eq!(f.value(), 4.);
/// assert_eq!(f.deriv(), 4.);
/// ```
pub type DualNumber = Common<f64>;

impl DualNumber {
    /// Construct a variable
    #[must_use]
    pub fn variable(real: f64) -> Self {
        Self::new(real, 1f64)
    }
    /// Get the derivative (dual component)
    #[must_use]
    pub fn deriv(&self) -> f64 {
        self.dual()
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

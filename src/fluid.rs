//! [`Dual`] trait as behavior definition

use ergo_traits::IntoErg;
use num_traits::{real::Real, Num, One, Zero};
use std::ops::Mul;

// NOTE: foreign traits (e.g. `std::ops::*`) can be implemented for solid structs only,
// TODO: but I can bound this trait correspondingly
// FIXME: setup bounds within a separate trait for Dual::Grad
/// Common behavior of dual numbers
pub trait Dual
where
    Self: Sized,
    for<'a, 'b> &'a Self::Grad: Mul<&'b Self::Value, Output = Self::Grad>,
{
    /// An ordinary real value
    type Value: Real + From<i32>;

    /// Borrow [`Dual::Value`]
    fn value(&self) -> &Self::Value;

    /// A storage for partial derivatives
    type Grad: Mul<f64, Output = Self>; // FIXME: setup bounds within a separate trait for Dual::Grad

    /// Borrow [`Dual::Grad`]
    fn grad(&self) -> &Self::Grad;

    /// Construct a new [`Dual`] from its parts
    fn new(value: &Self::Value, grad: &Self::Grad) -> Self;

    /// Chain rule implementation
    #[must_use]
    fn chain<F>(&self, func: F) -> Self
    where
        F: Fn(&Self::Value) -> (Self::Value, Self::Value),
    {
        let (f, df) = func(self.value());
        Self::new(&f, &self.grad().mul(&df))
    }

    /// Differentiable [`Real::powf`]
    #[must_use]
    fn powf(&self, exp: Self::Value) -> Self {
        self.chain(|x: &Self::Value| (x.powf(exp), x.powf(exp - Self::Value::one()) * exp))
    }

    /// Differentiable [`Real::powi`]
    #[must_use]
    fn powi<N: Num>(&self, exp: i32) -> Self {
        self.chain(|x| (x.powi(exp), x.powi(exp - 1) * exp.into_erg()))
    }

    /// Differentiable [`Real::sin`]
    #[must_use]
    fn sin(&self) -> Self {
        let (sin, cos) = self.value().sin_cos();
        self.chain(|_| (sin, cos))
    }

    /// Differentiable [`Real::cos`]
    #[must_use]
    fn cos(&self) -> Self {
        let (sin, cos) = self.value().sin_cos();
        self.chain(|_| (cos, -sin))
    }

    /// Differentiable [`Real::sin_cos`]
    fn sin_cos(&self) -> (Self, Self) {
        todo!()
    }

    /// Differentiable [`Real::exp`]
    #[must_use]
    fn exp(&self) -> Self {
        let real = self.value().exp();
        self.chain(|_| (real, real))
    }

    /// Differentiable [`Real::ln`]
    #[must_use]
    fn ln(&self) -> Self {
        self.chain(|x| (x.ln(), x.recip()))
    }

    /// Differentiable reciprocal of [`Dual`]
    #[must_use]
    fn recip(&self) -> Self {
        todo!()
    }

    /// Differentiable [`Real::abs`]
    #[must_use]
    fn abs(&self) -> Self {
        self.chain(|x| (x.abs(), x.signum()))
    }

    /// Differentiable [`Real::signum`]
    #[must_use]
    fn signum(&self) -> Self {
        self.chain(|x| (x.signum(), Self::Value::zero()))
    }
}

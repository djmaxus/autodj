//! [`Dual`] trait as behavior definition

use num_traits::{real::Real, One, Zero};
use std::ops::MulAssign;

/// An ordinary Value
pub trait Value: Real {}
impl<R: Real> Value for R {}

/// Derivatives
pub trait Grad<V: Value>
where
    Self: Clone + MulAssign<V>,
{
}
impl<V: Value, G> Grad<V> for G where G: Clone + MulAssign<V> {}

// NOTE: foreign traits (e.g. `std::ops::*`) can be implemented for solid structs only,
// TODO: but I can bound this trait correspondingly
// FIXME: naming conventions for real and dual components
// TODO: generic impls of arithmetic operations
// TODO: arithmetics on borrowed values
/// Common behavior of dual numbers
pub trait Dual
where
    Self: Sized,
{
    /// Associated [`Value`] implementor
    type Value: Value;

    /// Borrow [`Dual::Value`]
    fn value(&self) -> Self::Value;

    /// Associated [`Grad`] implementor
    type Grad: Grad<Self::Value>;

    /// Borrow [`Dual::Grad`]
    fn dual(&self) -> &Self::Grad;

    /// Mutably borrow [`Dual::Grad`]
    fn dual_mut(&mut self) -> &mut Self::Grad;

    /// Construct a new [`Dual`] from its parts
    fn new(value: Self::Value, grad: Self::Grad) -> Self;

    /// Chain rule implementation
    /// [`Fn(f64) -> (f64, f64)`] evaluates both function and its derivative
    #[must_use]
    fn chain<F>(&self, func: F) -> Self
    where
        F: Fn(Self::Value) -> (Self::Value, Self::Value),
    {
        let (f, df) = func(self.value());
        let dual_new = {
            let mut dual = self.dual().clone();
            dual *= df;
            dual
        };
        Self::new(f, dual_new)
    }

    /// Differentiable [`Real::powf`]
    #[must_use]
    #[inline]
    fn powf(&self, exp: Self::Value) -> Self {
        self.chain(|x: Self::Value| (x.powf(exp), x.powf(exp - Self::Value::one()) * exp))
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

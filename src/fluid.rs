//! [`Dual`] trait as behavior definition

use num_traits::{real::Real, One, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// An ordinary Value
pub trait Value: Real + AddAssign + MulAssign + SubAssign {}
impl<R> Value for R where R: Real + AddAssign + MulAssign + SubAssign {}

/// Derivatives
pub trait Grad<V: Value>
where
    Self: Clone
        + AddAssign
        + SubAssign
        + Neg<Output = Self>
        + MulAssign<V>
        + Mul<V, Output = Self>
        + PartialEq
        + PartialOrd,
{
}
impl<V: Value, G> Grad<V> for G where
    G: Clone
        + AddAssign
        + SubAssign
        + Neg<Output = Self>
        + MulAssign<V>
        + Mul<V, Output = Self>
        + PartialEq
        + PartialOrd
{
}

// TODO: implement parameters and independent variables there
// TODO: consistent naming convention for real and dual components (for both fluid & solid)
// TODO: std::ops::Index(Mut) ?
// TODO: reduce cloning
// TODO: find ways to reduce boilerplate
// NOTE: foreign traits (e.g. `std::ops::*`) can be implemented for solid structs only.
// That's why we have separate implementations down below + trait bounds right below
/// Common behavior of dual numbers
pub trait Dual
where
    Self: Sized
        + Clone
        + PartialEq
        + PartialOrd
        + Add<Output = Self>
        + Mul<Output = Self>
        + Sub<Output = Self>
        + Div<Output = Self>
        + AddAssign
        + DivAssign
        + MulAssign
        + SubAssign
        + Neg,
{
    /// Associated [`Value`] implementor
    type Value: Value;

    /// Copy [`Dual::Value`]
    fn value(&self) -> &Self::Value;

    /// Mutate [`Dual::Value`]
    fn value_mut(&mut self) -> &mut Self::Value;

    /// Associated [`Grad`] implementor
    type Grad: Grad<Self::Value>;

    /// Consume [`Dual`] implementor and return its components as a tuple
    fn decompose(self) -> (Self::Value, Self::Grad);

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
        let (f, df) = func(*self.value());
        let dual_new = {
            let mut dual = self.dual().to_owned();
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
        self.sin_cos().0 // TODO: check if the other tuple member is optimized out
    }

    /// Differentiable [`Real::cos`]
    #[must_use]
    fn cos(&self) -> Self {
        self.sin_cos().1 // TODO: check if the other tuple member is optimized out
    }

    /// Differentiable [`Real::sin_cos`]
    fn sin_cos(&self) -> (Self, Self) {
        let (sin, cos) = self.value().sin_cos();
        (self.chain(|_| (sin, cos)), self.chain(|_| (cos, -sin)))
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
        self.powf(-Self::Value::one())
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

    /// To further implement [`std::ops::Add`] for structs
    #[must_use]
    fn add_impl(&self, rhs: &Self) -> Self {
        self.to_owned().add_assign_impl(rhs).to_owned()
    }

    /// To further implement [`std::ops::Mul`] for structs
    #[must_use]
    fn mul_impl(&self, rhs: &Self) -> Self {
        self.to_owned().mul_assign_impl(rhs).to_owned()
    }

    /// To further implement [`std::ops::Sub`] for structs
    #[must_use]
    fn sub_impl(&self, rhs: &Self) -> Self {
        self.to_owned().sub_assign_impl(rhs).to_owned()
    }

    /// To further implement [`std::ops::Div`] for structs
    #[must_use]
    fn div_impl(&self, rhs: &Self) -> Self {
        self.to_owned().div_assign_impl(rhs).to_owned()
    }

    /// To further implement [`std::ops::AddAssign`] for structs
    fn add_assign_impl(&mut self, rhs: &Self) -> &mut Self {
        *self.value_mut() += rhs.value().to_owned();
        *self.dual_mut() += rhs.dual().to_owned();
        self
    }

    /// To further implement [`std::ops::MulAssign`] for structs
    fn mul_assign_impl(&mut self, rhs: &Self) -> &mut Self {
        let value_local = self.value().to_owned(); // preserve original value
        *self.value_mut() *= rhs.value().to_owned();
        *self.dual_mut() *= rhs.value().to_owned();
        *self.dual_mut() += rhs.dual().to_owned() * value_local;
        self
    }

    /// To further implement [`std::ops::SubAssign`] for structs
    fn sub_assign_impl(&mut self, rhs: &Self) -> &mut Self {
        self.add_assign_impl(&rhs.neg_impl())
    }

    /// To further implement [`std::ops::DivAssign`] for structs
    fn div_assign_impl(&mut self, rhs: &Self) -> &mut Self {
        self.mul_assign_impl(&rhs.recip())
    }

    /// To further implement [`std::ops::Neg`] for structs
    #[must_use]
    fn neg_impl(&self) -> Self {
        Self::new(self.value().to_owned().neg(), self.dual().to_owned().neg())
    }
}

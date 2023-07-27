//! [`crate::array::DualNumber`] for a specific number of variables

pub use crate::solid::*;
use num_traits::Zero;
use std::{
    array::from_fn,
    fmt::LowerExp,
    ops::{Add, AddAssign, Mul, MulAssign, Neg},
};

/// Array of dual components
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Grad<V: Value, const N: usize>([V; N]);

impl<V: Value, const N: usize> AsRef<[V; N]> for Grad<V, N> {
    fn as_ref(&self) -> &[V; N] {
        &self.0
    }
}

impl<V: Value, const N: usize, Arr: Into<[V; N]>> From<Arr> for Grad<V, N> {
    fn from(value: Arr) -> Self {
        Self(value.into())
    }
}

impl<V: Value, const N: usize> AddAssign for Grad<V, N> {
    fn add_assign(&mut self, rhs: Self) {
        for (index, elem) in self.0.iter_mut().enumerate() {
            // TODO: consider using `unsafe get_unchecked()` or relax clippy lints
            // ```
            // let value = unsafe { rhs.0.get_unchecked(index) }.to_owned();
            // ```
            if let Some(&value) = rhs.0.get(index) {
                *elem += value;
            } else {
                panic!("Index {index} should be valid");
            }
        }
    }
}

impl<V: Value, const N: usize> MulAssign<V> for Grad<V, N> {
    fn mul_assign(&mut self, rhs: V) {
        for elem in self.0.iter_mut() {
            *elem *= rhs;
        }
    }
}

impl<V: Value, const N: usize> Mul<V> for Grad<V, N> {
    type Output = Self;

    fn mul(self, rhs: V) -> Self::Output {
        let mut out = self;
        out.mul_assign(rhs);
        out
    }
}

impl<V: Value, const N: usize> Add for Grad<V, N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut out = self;
        out.add_assign(rhs);
        out
    }
}

impl<V: Value, const N: usize> Neg for Grad<V, N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut out: Self = self;
        for elem in out.0.iter_mut() {
            *elem = elem.neg();
        }
        out
    }
}

impl<V, const N: usize> Zero for Grad<V, N>
where
    V: Value,
{
    fn zero() -> Self {
        Grad([V::zero(); N])
    }

    fn is_zero(&self) -> bool {
        let non_zero_element = self.0.iter().find(|elem| !elem.is_zero());
        non_zero_element.is_some()
    }
}

/// For statically-known number of variables
///```
/// use autodj::array::*;
/// let x0 : DualNumber<f64,2> = 1.0.into(); // Parameter
/// let [x, y] = [2.,3.].into_variables();
/// let f = (x - x0) * y;
/// assert_eq!(f.value(), &3.);
/// assert_eq!(f.dual().as_ref().len(), 2);
/// ```
pub type DualNumber<V, const N: usize> = crate::solid::DualNumber<V, Grad<V, N>>;

/// Construct independent variables from array
pub trait IntoVariables<V: Value, const N: usize>: Into<[V; N]> {
    /// Construct independent variables from array
    fn into_variables(self) -> [DualNumber<V, N>; N] {
        let arr: [V; N] = self.into();
        from_fn(|index| {
            let grad: [V; N] = from_fn(|grad_index| {
                if grad_index == index {
                    V::one()
                } else {
                    V::zero()
                }
            });
            DualNumber::new(
                arr.get(index)
                    // TODO: consider using `unsafe get_unchecked()` or relax clippy lints
                    .unwrap_or_else(|| panic!(r#"This index "{index}" should be valid"#))
                    // TODO: consider consuming input array `[V; N]` at some point to avoid copies
                    .to_owned(),
                Grad(grad),
            )
        })
    }
}
impl<V: Value, const N: usize, IntoArray> IntoVariables<V, N> for IntoArray where Self: Into<[V; N]> {}

impl<V: Value, const N: usize> Display for Grad<V, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+{:?}", self.0)
    }
}

impl<V: Value + LowerExp, const N: usize> LowerExp for Grad<V, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+[")?;
        for index in 1..=N {
            let deriv_value = self
                .0
                .get(index - 1)
                // TODO: consider using `unsafe get_unchecked()` or relax clippy lints
                .unwrap_or_else(|| panic!("The index requested here should always be valid"));
            write!(f, "{deriv_value:e}")?;
            if index == N {
                break;
            }
            write!(f, ", ")?;
        }
        write!(f, "]")
    }
}

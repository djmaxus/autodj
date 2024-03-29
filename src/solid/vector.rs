//! [`crate::vector::DualNumber`] for dynamic number of variables

use crate::fluid::{Dual, Value};
use num_traits::Zero;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg};
use std::prelude::v1::*;

/// Newtype wrapper for [`Vec<impl crate::fluid::Value>`].
/// Should implement [`crate::fluid::Grad`]

#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Grad<V: Value>(Vec<V>);

impl<V: Value> AsRef<Vec<V>> for Grad<V> {
    fn as_ref(&self) -> &Vec<V> {
        &self.0
    }
}

impl<V: Value, IntoVec: Into<Vec<V>>> From<IntoVec> for Grad<V> {
    fn from(value: IntoVec) -> Self {
        Self(value.into())
    }
}

/// For *dynamically*-known number of variables
pub type DualNumber<V> = crate::solid::DualNumber<V, Grad<V>>;

impl<V: Value> AddAssign for Grad<V> {
    fn add_assign(&mut self, rhs: Self) {
        self.0.resize_with(self.0.len().max(rhs.0.len()), V::zero);
        for (to, from) in self.0.iter_mut().zip(rhs.0.into_iter()) {
            *to += from;
        }
    }
}

impl<V: Value> MulAssign<V> for Grad<V> {
    fn mul_assign(&mut self, rhs: V) {
        for elem in &mut self.0 {
            *elem *= rhs;
        }
    }
}

impl<V: Value> Mul<V> for Grad<V> {
    type Output = Self;

    fn mul(self, rhs: V) -> Self::Output {
        let mut result = self;
        result *= rhs;
        result
    }
}

impl<V: Value> Neg for Grad<V> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.0
            .into_iter()
            .map(std::ops::Neg::neg)
            .collect::<Vec<V>>()
            .into()
    }
}

impl<V: Value> Add for Grad<V> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

impl<V: Value> Zero for Grad<V> {
    fn zero() -> Self {
        Self(Vec::new())
    }

    fn is_zero(&self) -> bool {
        let non_zero_element = self.0.iter().find(|elem| !elem.is_zero());
        non_zero_element.is_some()
    }
}

/// Turn [`Vec`] of [`crate::fluid::Value`] into independent dual variables
pub trait IntoVariables<V: Value>: Into<Vec<V>> {
    /// Turn [`Vec`] of [`crate::fluid::Value`] into independent dual variables
    fn into_variables(self) -> Vec<DualNumber<V>> {
        let vec: Vec<V> = self.into();
        let len = vec.len();
        let mut result = Vec::<DualNumber<V>>::with_capacity(len);
        // preallocate gradients with known capacity
        let grads_holder = {
            let grad_holder = vec![V::zero(); len];
            vec![grad_holder; len]
        };
        for (index, (mut grad, value)) in grads_holder.into_iter().zip(vec.into_iter()).enumerate()
        {
            *grad
                .get_mut(index)
                // TODO: consider using `unsafe get_unchecked()` or relax clippy lints
                .unwrap_or_else(|| {
                    panic!("The index requested here should be valid at this point")
                }) = V::one();
            result.push(Dual::new(value, grad.into()));
        }
        result
    }
}
impl<V: Value, IntoVec> IntoVariables<V> for IntoVec where IntoVec: Into<Vec<V>> {}

// TODO: implement in other similar places
/// Specialization for [`f64`]
pub type DualF64 = DualNumber<f64>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn div() {
        let a = Grad(vec![1.0_f64, 2.0_f64, 3.0_f64]);
        let b = a * 2.0_f64.recip();
        assert_eq!(b, Grad(vec![0.5_f64, 1.0_f64, 1.5_f64]));
    }

    #[test]
    fn neg() {
        let a = Grad(vec![-1.0_f64, 2.0_f64, 0.0_f64]);
        let b = -a;
        assert_eq!(b, Grad(vec![1.0_f64, -2.0_f64, 0.0_f64]));
    }

    #[test]
    fn add_assign() {
        let a = Grad(vec![1.0_f64, 2.0_f64, 3.0_f64]);
        let mut b = Grad(vec![0.5_f64, 1.0_f64]);
        b += a;
        assert_eq!(b, Grad(vec![1.5_f64, 3.0_f64, 3.0_f64]));
    }
}

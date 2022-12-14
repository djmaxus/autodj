//! [`crate::vector::DualNumber`] for dynamic number of variables

/// Specialization for dynamic number of variables
pub type DualNumber = crate::common::Common<Vector>;

/// Dynamic dense dual component
#[derive(Clone, Debug, PartialEq)]
pub struct Vector(Vec<f64>);

impl DualNumber {
    /// Refer to the gradient (dual component)
    #[must_use]
    pub fn grad(&self) -> &Vec<f64> {
        &self.dual.0
    }
}

impl DualComponent for Vector {
    fn zero() -> Self {
        Self(Vec::new())
    }
}

/// Dynamic dual variables
pub struct DualVariables {
    variables: Vec<DualNumber>,
}

impl DualVariables {
    /// Access dual variables
    #[must_use]
    pub fn get(&self) -> &Vec<DualNumber> {
        &self.variables
    }
    /// Evaluate a function with contained dual variables
    pub fn eval<Out>(&self, func: impl Fn(&Vec<DualNumber>) -> Out) -> Out {
        func(self.get())
    }
    fn new(values: &[f64]) -> Self {
        let mut variables: Vec<DualNumber> = values.iter().map(|&x| x.into()).collect();
        variables.iter_mut().enumerate().for_each(|(i, x)| {
            x.dual.0.resize(i + 1, 0.);
            x.dual.0[i] = 1.;
        });
        Self { variables }
    }
}

impl From<&[f64]> for DualVariables {
    fn from(values: &[f64]) -> Self {
        Self::new(values)
    }
}

/// Convenient wrapper for Into<DualVariables>
pub trait IntoVariables {
    /// Convert into dual variables
    fn into_variables(self) -> DualVariables;
}

impl IntoVariables for &[f64] {
    fn into_variables(self) -> DualVariables {
        self.into()
    }
}

impl Add for &Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        let mut out = self.clone();
        out += rhs;
        out
    }
}

impl Sub for &Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = self.clone();
        out -= rhs;
        out
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = self;
        out -= rhs;
        out
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut out = self;
        out *= rhs;
        out
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut out = self.clone();
        out *= rhs;
        out
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        self.0.iter_mut().for_each(|x| *x *= rhs);
    }
}

impl<'a> Sum<&'a Self> for DualNumber {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let init = 0.0.into();
        let f = |acc, x: &DualNumber| acc + x;
        iter.fold(init, f)
    }
}

impl Sum for DualNumber {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let init = 0.0.into();
        let f = |acc, x| acc + x;
        iter.fold(init, f)
    }
}

impl<'a> Product<&'a Self> for DualNumber {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let init = 1.0.into();
        let f = |acc, x: &DualNumber| acc * x;
        iter.fold(init, f)
    }
}

impl Product<Self> for DualNumber {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        let init = 1.0.into();
        let f = |acc, x: DualNumber| acc * x;
        iter.fold(init, f)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Self::Output {
        Vector(self.0.iter().map(|x| x / rhs).collect())
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector(self.0.iter().map(|x| -x).collect())
    }
}

impl AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, rhs: &Self) {
        let new_len = self.0.len().max(rhs.0.len());

        self.0.resize(new_len, 0.0);

        rhs.0
            .iter()
            .zip(self.0.iter_mut())
            .for_each(|(term, destination)| *destination += term);
    }
}

impl SubAssign<&Vector> for Vector {
    fn sub_assign(&mut self, rhs: &Self) {
        let new_len = self.0.len().max(rhs.0.len());

        self.0.resize(new_len, 0.0);

        rhs.0
            .iter()
            .zip(self.0.iter_mut())
            .for_each(|(term, destination)| *destination -= term);
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        let new_len = self.0.len().max(rhs.0.len());

        self.0.resize(new_len, 0.0);

        rhs.0
            .iter()
            .zip(self.0.iter_mut())
            .for_each(|(term, destination)| destination.sub_assign(term));
    }
}

use crate::common::DualComponent;

use std::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn div() {
        let a = Vector(vec![1., 2., 3.]);
        let b = a / 2.;
        assert_eq!(b, Vector(vec![0.5, 1., 1.5]));
    }

    #[test]
    fn neg() {
        let a = Vector(vec![-1., 2., 0.]);
        let b = -a;
        assert_eq!(b, Vector(vec![1., -2., 0.]));
    }

    #[test]
    fn add_assign() {
        let a = Vector(vec![1., 2., 3.]);
        let mut b = Vector(vec![0.5, 1., 1.5]);
        b += a;
        assert_eq!(b, Vector(vec![1.5, 3.0, 4.5]));
    }
}

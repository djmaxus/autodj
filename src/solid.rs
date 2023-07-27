//! Generic data structure which implements [`Dual`]

use std::{
    borrow::BorrowMut,
    fmt::{Display, LowerExp},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::fluid::{display_impl, lower_exp_impl, Dual, Grad, Value};

/// Default generic [`Dual`] implementor: a struct with two fields
#[derive(Clone, Debug, PartialEq, PartialOrd, Default, Hash)]
pub struct DualNumber<N, D>
where
    N: Value,
    D: Grad<N>,
{
    /// actual value
    value: N,
    /// derivatives (dual components)
    dual: D,
}

impl<N, D> Neg for DualNumber<N, D>
where
    N: Value,
    D: Grad<N>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.neg_impl()
    }
}

impl<N, D> SubAssign for DualNumber<N, D>
where
    N: Value,
    D: Grad<N>,
{
    fn sub_assign(&mut self, rhs: Self) {
        let _unused = self.sub_assign_impl(&rhs);
    }
}

impl<N, D> MulAssign for DualNumber<N, D>
where
    N: Value,
    D: Grad<N>,
{
    fn mul_assign(&mut self, rhs: Self) {
        let _unused = self.mul_assign_impl(&rhs);
    }
}

impl<N, D> DivAssign for DualNumber<N, D>
where
    N: Value,
    D: Grad<N>,
{
    fn div_assign(&mut self, rhs: Self) {
        let _unused = self.div_assign_impl(&rhs);
    }
}

impl<N, D> AddAssign for DualNumber<N, D>
where
    N: Value,
    D: Grad<N>,
{
    fn add_assign(&mut self, rhs: Self) {
        let _unused = self.add_assign_impl(&rhs);
    }
}

impl<N, D> Div for DualNumber<N, D>
where
    N: Value,
    D: Grad<N>,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.div_impl(&rhs)
    }
}

impl<N, D> Sub for DualNumber<N, D>
where
    N: Value,
    D: Grad<N>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_impl(&rhs)
    }
}

impl<N, D> Mul for DualNumber<N, D>
where
    N: Value,
    D: Grad<N>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_impl(&rhs)
    }
}

impl<N, D> Add for DualNumber<N, D>
where
    N: Value,
    D: Grad<N>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_impl(&rhs)
    }
}

impl<N, D> Dual for DualNumber<N, D>
where
    N: Value,
    D: Grad<N>,
{
    type Value = N;

    fn value(&self) -> &Self::Value {
        &self.value
    }

    fn value_mut(&mut self) -> &mut Self::Value {
        self.value.borrow_mut()
    }

    type Grad = D;

    fn dual(&self) -> &Self::Grad {
        &self.dual
    }

    fn dual_mut(&mut self) -> &mut Self::Grad {
        self.dual.borrow_mut()
    }

    fn new(real: Self::Value, dual: Self::Grad) -> Self {
        Self { value: real, dual }
    }

    fn decompose(self) -> (Self::Value, Self::Grad) {
        (self.value, self.dual)
    }
}

impl<V: Value, G: Grad<V>> From<V> for DualNumber<V, G> {
    fn from(value: V) -> Self {
        Self::parameter(value)
    }
}

impl<V: Value + Display, G: Grad<V> + Display> Display for DualNumber<V, G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        display_impl(self, f)
    }
}

impl<V: Value + LowerExp, G: Grad<V> + LowerExp> LowerExp for DualNumber<V, G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        lower_exp_impl(self, f)
    }
}

impl<N, D> Copy for DualNumber<N, D>
where
    N: Value + Copy,
    D: Grad<N> + Copy,
{
}

pub mod array;
pub mod single;
pub mod sparse;
pub mod vector;

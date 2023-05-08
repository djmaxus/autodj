//! Generic data structure which implements [`Dual`]

use crate::fluid::{Dual, Grad, Value};
use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

// TODO: rebase specializations on this generic struct
// TODO: name specifications something like "Song"(single), "Album"(static arr), "Playlist"(vector),"Mixtape"(sparse) | optional re-export feature
/// Default generic [`Dual`] implementor: a struct with two fields
#[derive(Clone, Debug, PartialEq, PartialOrd, Default, Hash)]
pub struct DualNumber<N, D>
where
    N: Value,
    D: Grad<N>,
{
    /// real value
    real: N,
    /// derivatives
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

    fn value(&self) -> Self::Value {
        self.real
    }

    fn value_mut(&mut self) -> &mut Self::Value {
        &mut self.real
    }

    type Grad = D;

    fn dual(&self) -> Self::Grad {
        self.dual.to_owned()
    }

    fn dual_borrow(&self) -> &Self::Grad {
        self.dual.borrow()
    }

    fn dual_mut(&mut self) -> &mut Self::Grad {
        self.dual.borrow_mut()
    }

    fn new(real: Self::Value, dual: Self::Grad) -> Self {
        Self { real, dual }
    }
}

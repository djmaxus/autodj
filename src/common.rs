//! Common definitions of dual arithmetics

pub use crate::fluid::Dual;
use std::{
    fmt::{Display, LowerExp},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// Common structure of dual numbers
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Common<D: DualComponent> {
    /// Ordinary value
    real: f64,
    /// Derivatives
    dual: D,
}

impl<D: DualComponent> Dual for Common<D> {
    type Value = f64;

    fn value(&self) -> Self::Value {
        self.real
    }

    type Grad = D;

    fn dual(&self) -> &Self::Grad {
        &self.dual
    }

    fn new(value: Self::Value, dual: Self::Grad) -> Self {
        Self { real: value, dual }
    }

    fn dual_mut(&mut self) -> &mut Self::Grad {
        &mut self.dual
    }
}

// FIXME: move to `fluid.rs` and refactor
// TODO: test `Default` implementations
/// Requirements for dual component
pub trait DualComponent: Sized + Clone + PartialEq + PartialOrd + Default
where
    Self: Add<Self, Output = Self>
        + Sub<Self, Output = Self>
        + Mul<f64, Output = Self>
        + Div<f64, Output = Self>
        + Neg<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign<f64>,
{
}

impl<D> DualComponent for D where
    D: Sized
        + Clone
        + PartialEq
        + PartialOrd
        + Default
        + Add<Output = D>
        + Sub<Output = D>
        + Neg<Output = D>
        + AddAssign
        + SubAssign
        + Mul<f64, Output = D>
        + Div<f64, Output = D>
        + MulAssign<f64>
{
}

impl<D: DualComponent> From<f64> for Common<D> {
    fn from(real: f64) -> Self {
        Self {
            real,
            dual: D::default(),
        }
    }
}

impl<D: DualComponent + Display> Display for Common<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:+}∆", self.real, self.dual)
    }
}

impl<D: DualComponent + LowerExp> LowerExp for Common<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:e}{:e}∆", self.real, self.dual)
    }
}

impl<D: DualComponent> Add for Common<D> {
    type Output = Common<D>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            real: self.value() + rhs.value(),
            dual: self.dual + rhs.dual,
        }
    }
}

impl<D: DualComponent> Mul for Common<D> {
    type Output = Common<D>;

    fn mul(self, rhs: Self) -> Self::Output {
        let real = self.real * rhs.real;
        let a = self.dual * rhs.real;
        let b = rhs.dual * self.real;
        let dual = a + b;
        Self::Output { real, dual }
    }
}

impl<D: DualComponent> Sub for Common<D> {
    type Output = Common<D>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            real: self.real - rhs.real,
            dual: self.dual - rhs.dual,
        }
    }
}

impl<D: DualComponent> Div for Common<D> {
    type Output = Common<D>;

    fn div(self, rhs: Self) -> Self::Output {
        let real = self.real / rhs.real;

        let a = self.dual * rhs.real;
        let b = rhs.dual * self.real;
        let reciprocal_denominator = rhs.real.powi(-2);
        let dual = (a - b) * reciprocal_denominator;

        Self::Output { real, dual }
    }
}

impl<D: DualComponent> AddAssign for Common<D> {
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.dual += rhs.dual;
    }
}

impl<D: DualComponent> SubAssign for Common<D> {
    fn sub_assign(&mut self, rhs: Self) {
        self.real -= rhs.real;
        self.dual -= rhs.dual;
    }
}

impl<D: DualComponent> DivAssign for Common<D> {
    fn div_assign(&mut self, rhs: Self) {
        self.real /= rhs.real;

        let a = self.dual.clone() * rhs.real;
        let b = rhs.dual * self.real;
        let reciprocal_denominator = rhs.real.powi(-2);
        let dual = (a - b) * reciprocal_denominator;
        self.dual = dual;
    }
}

impl<D: DualComponent> MulAssign for Common<D> {
    fn mul_assign(&mut self, rhs: Self) {
        self.real *= rhs.real;
        self.dual *= rhs.real;
        self.dual += rhs.dual * self.real;
    }
}

impl<D: DualComponent> Neg for Common<D> {
    type Output = Common<D>;

    fn neg(self) -> Self::Output {
        Self::Output {
            real: -self.real,
            dual: -self.dual,
        }
    }
}

/// Basic arithmetic operations for references to [`DualCommon`]
pub mod ops_ref {
    use super::{Add, Common, Div, DualComponent, Mul, Sub};
    use crate::fluid::Dual;

    impl<D: DualComponent> Add for &Common<D>
    where
        for<'a> &'a D: Add<Output = D>,
    {
        type Output = Common<D>;
        fn add(self, rhs: Self) -> Self::Output {
            Self::Output {
                real: self.value() + rhs.value(),
                dual: &self.dual + &rhs.dual,
            }
        }
    }

    impl<D: DualComponent> Mul for &Common<D>
    where
        for<'a> &'a D: Mul<f64, Output = D>,
    {
        type Output = Common<D>;

        fn mul(self, rhs: Self) -> Self::Output {
            let real = self.real * rhs.real;
            let dual = {
                let dx_y = &self.dual * rhs.real;
                let x_dy = &rhs.dual * self.real;
                dx_y + x_dy
            };
            Self::Output { real, dual }
        }
    }

    impl<D: DualComponent> Sub for &Common<D>
    where
        for<'a> &'a D: Sub<Output = D>,
    {
        type Output = Common<D>;

        fn sub(self, rhs: Self) -> Self::Output {
            Self::Output {
                real: self.real - rhs.real,
                dual: &self.dual - &rhs.dual,
            }
        }
    }

    impl<D: DualComponent> Div for &Common<D>
    where
        for<'a> &'a D: Mul<f64, Output = D>,
    {
        type Output = Common<D>;

        fn div(self, rhs: Self) -> Self::Output {
            let real = self.real / rhs.real;
            let dual = {
                let dx_y = &self.dual * rhs.real;
                let x_dy = &rhs.dual * self.real;
                let reciprocal_denominator = 1f64 / (rhs.real * rhs.real);
                (dx_y - x_dy) * reciprocal_denominator
            };
            Self::Output { real, dual }
        }
    }

    impl<D: DualComponent> Add<&Common<D>> for Common<D>
    where
        for<'a> &'a D: Add<Output = D>,
    {
        type Output = Common<D>;
        fn add(self, rhs: &Self) -> Self::Output {
            &self + rhs
        }
    }

    impl<D: DualComponent> Mul<&Common<D>> for Common<D>
    where
        for<'a> &'a D: Mul<f64, Output = D>,
    {
        type Output = Common<D>;

        fn mul(self, rhs: &Self) -> Self::Output {
            &self * rhs
        }
    }

    impl<D: DualComponent> Sub<&Common<D>> for Common<D>
    where
        for<'a> &'a D: Sub<Output = D>,
    {
        type Output = Common<D>;

        fn sub(self, rhs: &Self) -> Self::Output {
            &self - rhs
        }
    }

    impl<D: DualComponent> Div<&Common<D>> for Common<D>
    where
        for<'a> &'a D: Mul<f64, Output = D>,
    {
        type Output = Common<D>;

        fn div(self, rhs: &Self) -> Self::Output {
            &self / rhs
        }
    }
}

impl<T> Copy for Common<T> where T: DualComponent + Copy {}

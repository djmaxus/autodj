//! Common definitions of dual arithmetics

/// Common structure of dual numbers
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Common<D: DualComponent> {
    pub(crate) real: f64,
    pub(crate) dual: D,
}

impl<D: DualComponent> Common<D> {
    /// Return the real component
    pub fn value(&self) -> f64 {
        self.real
    }
}

/// Requirements for dual component
pub trait DualComponent: Sized + Clone + PartialEq
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
    /// Zero (or just empty) dual component
    fn zero() -> Self;
}

impl<D: DualComponent> From<f64> for Common<D> {
    fn from(real: f64) -> Self {
        Self {
            real,
            dual: D::zero(),
        }
    }
}

impl<D: DualComponent + Display> Display for Common<D>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:+}∆", self.real, self.dual)
    }
}

impl<D: DualComponent + LowerExp> LowerExp for Common<D>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:e}{:e}∆", self.real, self.dual)
    }
}

use std::{
    fmt::{Display, LowerExp},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

impl<D: DualComponent + Clone> Common<D>
{
    /// Chain rule implementation
    /// [`Fn(f64) -> (f64, f64)`] evaluates both function and its derivative
    #[must_use]
    pub fn chain<F>(&self, func: F) -> Self
    where
        F: Fn(f64) -> (f64, f64),
    {
        let (f, df) = func(self.real);
        Self {
            real: f,
            dual: self.dual.clone() * df,
        }
    }

    /// Raise to a floating-point power
    #[must_use]
    pub fn powf(&self, exp: f64) -> Self {
        self.chain(|x| (x.powf(exp), x.powf(exp - 1.) * exp))
    }

    /// Raise to an integer power
    #[must_use]
    pub fn powi(&self, exp: i32) -> Self {
        self.chain(|x| (x.powi(exp), x.powi(exp - 1) * f64::from(exp)))
    }

    /// Sine
    /// ```
    /// use autodj::common::*;
    /// use autodj::single::*;
    /// let x = 0.0.into_variable().sin();
    /// assert!((x.value() - 0.0).abs() <= f64::EPSILON);
    /// assert_eq!(x.deriv(), 1.0);
    /// ```
    #[must_use]
    pub fn sin(&self) -> Self {
        let (sin, cos) = self.real.sin_cos();
        self.chain(|_| (sin, cos))
    }

    /// Cosine
    /// ```
    /// # use autodj::common::*;
    /// use autodj::single::*;
    /// let x = core::f64::consts::FRAC_PI_2.into_variable().cos();
    /// assert!((x.value() - 0.0).abs() <= f64::EPSILON);
    /// assert_eq!(x.deriv(), -1.0);
    /// ```
    #[must_use]
    pub fn cos(&self) -> Self {
        let (sin, cos) = self.real.sin_cos();
        self.chain(|_| (cos, -sin))
    }

    /// Exponent
    #[must_use]
    pub fn exp(&self) -> Self {
        let real = self.real.exp();
        self.chain(|_| (real, real))
    }

    /// Natural logarithm
    #[must_use]
    pub fn ln(&self) -> Self {
        self.chain(|x| (x.ln(), x.recip()))
    }

    /// Absolute value
    #[must_use]
    pub fn abs(&self) -> Self {
        self.chain(|x| (x.abs(), x.signum()))
    }

    /// Sign function
    #[must_use]
    pub fn signum(&self) -> Self {
        self.chain(|x| (x.signum(), 0.))
    }

    /// Reciprocal
    #[must_use]
    pub fn recip(&self) -> Self {
        const UNIT: f64 = 1.0;
        let unit: Self = UNIT.into();
        unit / self.clone()
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
                let reciprocal_denominator = 1.0 / (rhs.real * rhs.real);
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

//! # Automatic Differentiation Library
//!
//! by [djmaxus](https://djmaxus.github.io/)
//!
//! ## Examples
//! - `ideal_gas`: use Newton method on dual numbers to find pressure consistent to ideal gas thermodynamic model
//!
//! ## References
//!
//! - [Wikipedia](https://en.wikipedia.org/wiki/Dual_number)

use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

/// Dual numbers as mathematical basis
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DualNumber {
    /// Ordinary ("real") component
    pub val: f64,
    /// Dual component
    pub dual: f64,
}

/// Construct autodifferentiation-specific dual numbers and evaluate functions over them
pub trait Dualize {
    /// Construct dual varable (with unit dual part)
    fn var(&self) -> DualNumber;

    /// Construct dual parameter (with zero dual part)
    fn par(&self) -> DualNumber;

    /// Apply `DualFunction` to a value treated as dual variable
    fn derive<DF>(&self, func: &DF) -> DualNumber
    where
        DF: DualFunction,
    {
        func(self.var())
    }
}

impl Dualize for f64 {
    fn var(&self) -> DualNumber {
        DualNumber {
            val: *self,
            dual: 1.,
        }
    }

    fn par(&self) -> DualNumber {
        DualNumber {
            val: *self,
            dual: 0.,
        }
    }
}

/// dual arithmetic
impl DualNumber {
    fn add(self, rhs: Self) -> Self {
        let val = self.val + rhs.val;
        let dual = self.dual + rhs.dual;
        Self { val, dual }
    }

    fn sub(self, rhs: Self) -> Self {
        let val = self.val - rhs.val;
        let dual = self.dual - rhs.dual;
        Self { val, dual }
    }

    fn mul(self, rhs: Self) -> Self {
        let val = self.val * rhs.val;
        let dual = self.dual * rhs.val + rhs.dual * self.val;
        Self { val, dual }
    }

    fn div(self, rhs: Self) -> Self {
        let val = self.val / rhs.val;
        let dual = (self.dual * rhs.val - rhs.dual * self.val) / (rhs.val * rhs.val);
        Self { val, dual }
    }

    fn neg(self) -> Self {
        Self {
            val: -self.val,
            dual: -self.dual,
        }
    }
}

impl Add for DualNumber {
    type Output = DualNumber;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

impl Sub for DualNumber {
    type Output = DualNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub(rhs)
    }
}

impl Mul for DualNumber {
    type Output = DualNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(rhs)
    }
}

impl Div for DualNumber {
    type Output = DualNumber;

    fn div(self, rhs: Self) -> Self::Output {
        self.div(rhs)
    }
}

impl Neg for DualNumber {
    type Output = DualNumber;

    fn neg(self) -> Self::Output {
        self.neg()
    }
}

/// Utility
impl DualNumber {
    /// Apply custom function-derivative pair to a dual number.
    /// May be used to extend the provided set of functions
    /// ```
    /// # use autodj::*;
    /// #
    /// trait OpsExtended{
    ///     fn powi(self, n: i32) -> Self;
    /// }
    ///
    /// impl OpsExtended for DualNumber {
    ///     fn powi(self, n: i32) -> Self {
    ///         self.custom(
    ///             &|x : f64| x.powi(n),//
    ///             &|x : f64| x.powi(n - 1) * (n as f64)
    ///         )
    ///     }
    /// }
    ///
    /// let x = std::f64::consts::PI;
    /// let n = 2;
    /// let f = x.var().powi(n);
    /// # assert_eq!(f,DualNumber{val:x.powi(n),dual:x.powi(n - 1) * (n as f64)});
    /// ```
    pub fn custom<F, D>(&self, func: &F, deriv: &D) -> Self
    where
        F: FloatFunction,
        D: FloatFunction,
    {
        Self {
            val: func(self.val),
            dual: self.dual * deriv(self.val),
        }
    }

    /// Apply `DualFunction` to a dual number
    pub fn derive<DF>(&self, func: &DF) -> Self
    where
        DF: DualFunction,
    {
        func(*self)
    }

    fn chain_of(self, val: f64, deriv: f64) -> Self {
        Self {
            val,
            dual: deriv * self.dual,
        }
    }
}

/// commonly used functions
impl DualNumber {
    pub fn powf(self, p: f64) -> Self {
        self.custom(
            &|x: f64| x.powf(p), //
            &|x: f64| x.powf(p - 1.) * p,
        )
    }

    pub fn sin(self) -> Self {
        let (sin, cos) = self.val.sin_cos();
        self.chain_of(sin, cos)
    }

    pub fn cos(self) -> Self {
        let (sin, cos) = self.val.sin_cos();
        self.chain_of(cos, -sin)
    }

    pub fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.val.sin_cos();
        (self.chain_of(sin, cos), self.chain_of(cos, -sin))
    }

    pub fn exp(self) -> Self {
        let val = self.val.exp();
        self.chain_of(val, val)
    }

    pub fn ln(self) -> Self {
        self.custom(
            &|x: f64| x.ln(), //
            &|x: f64| 1. / x,
        )
    }

    pub fn abs(self) -> Self {
        self.custom(
            &|x: f64| x.abs(), //
            &|x: f64| x.signum(),
        )
    }

    pub fn signum(self) -> Self {
        self.custom(
            &|x: f64| x.signum(), //
            &|_| 0.,
        )
    }
}

/// "Trait alias" for `Dual`-to-`Dual` functions
/// ```
/// # use autodj::*;
/// #
/// fn compose_dual_functions<DFnI, DFnII>(
///     df_i : &DFnI,
///     df_ii: &DFnII,
///       arg: &f64
/// ) -> DualNumber
/// where
///     DFnI  : DualFunction,
///     DFnII : DualFunction,
/// {
///     arg.derive(df_i).derive(df_ii)
/// }
///
/// let square   = |var| var * var;
/// let plus_one = |var| var + 1.0.par();
///
/// let x = 2.;
///
/// let y = compose_dual_functions(&square, &plus_one, &x);
/// # assert_eq!(y, DualNumber{val:x*x+1.,dual:2.*x});
/// ```
pub trait DualFunction: Fn(DualNumber) -> DualNumber {}
impl<F> DualFunction for F where F: Fn(DualNumber) -> DualNumber {}

/// "Trait alias" for `f64`-to-`f64` functions
pub trait FloatFunction: Fn(f64) -> f64 {}
impl<FF> FloatFunction for FF where FF: Fn(f64) -> f64 {}

// Unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}

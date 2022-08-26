// TODO: reorder items for reading purposes
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

/// Dual numbers as mathematical basis
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DualNumber {
    val: f64,
    dual: f64,
}

impl DualNumber {
    /// Ordinary ("real") component, just a value
    pub fn val(&self) -> &f64 {
        &self.val
    }
    /// Dual component, a derivative
    pub fn deriv(&self) -> &f64 {
        &self.dual
    }
    /// An arbitrary dual number
    pub fn new(val: f64, dual: f64) -> Self {
        Self { val, dual }
    }
}

/// Construct autodifferentiation-specific [`DualNumber`]s and evaluate functions over them
pub trait Dualize {
    /// Construct dual varable (with unit dual part)
    fn var(&self) -> DualNumber;

    /// Construct dual parameter (with zero dual part)
    fn par(&self) -> DualNumber;

    /// Apply [`DualFunction`] to a value treated as [`DualNumber`] variable
    fn eval<DF>(&self, func: &DF) -> DualNumber
    where
        DF: DualFunction,
    {
        func(&self.var())
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

impl Add for DualNumber {
    type Output = DualNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let val = self.val + rhs.val;
        let dual = self.dual + rhs.dual;
        Self { val, dual }
    }
}

impl Sub for DualNumber {
    type Output = DualNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        let val = self.val - rhs.val;
        let dual = self.dual - rhs.dual;
        Self { val, dual }
    }
}

impl Mul for DualNumber {
    type Output = DualNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        let val = self.val * rhs.val;
        let dual = self.dual * rhs.val + rhs.dual * self.val;
        Self { val, dual }
    }
}

impl Div for DualNumber {
    type Output = DualNumber;

    fn div(self, rhs: Self) -> Self::Output {
        let val = self.val / rhs.val;
        let dual = (self.dual * rhs.val - rhs.dual * self.val) / (rhs.val * rhs.val);
        Self { val, dual }
    }
}

impl Neg for DualNumber {
    type Output = DualNumber;

    fn neg(self) -> Self::Output {
        Self {
            val: -self.val,
            dual: -self.dual,
        }
    }
}

/// Utility
impl DualNumber {
    /// Apply custom function-derivative pair to a [`DualNumber`].\
    /// May be used to extend the provided set of functions
    /// ```
    /// # use autodj::single::*;
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
    /// # assert_eq!(f,DualNumber::new(x.powi(n), x.powi(n - 1) * (n as f64)));
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

    /// Apply [`DualFunction`] to a [`DualNumber`]
    pub fn eval<DF>(&self, func: &DF) -> Self
    where
        DF: DualFunction,
    {
        func(self)
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

/// "Trait alias" for transforms within [`DualNumber`]'s domain
/// ```
/// # use autodj::single::*;
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
///     arg.eval(df_i).eval(df_ii)
/// }
///
/// let square   = |var : &_| *var * *var;
/// let plus_one = |var : &_| *var + 1.0.par();
///
/// let x = 2.;
///
/// let y = compose_dual_functions(&square, &plus_one, &x);
/// # assert_eq!(y, DualNumber::new(x*x+1.,2.*x));
/// ```
pub trait DualFunction: Fn(&DualNumber) -> DualNumber {}
impl<F> DualFunction for F where F: Fn(&DualNumber) -> DualNumber {}

/// "Trait alias" for transforms within [`f64`]'s domain
pub trait FloatFunction: Fn(f64) -> f64 {}
impl<FF> FloatFunction for FF where FF: Fn(f64) -> f64 {}

// Unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}

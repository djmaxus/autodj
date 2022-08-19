use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

/// Dual numbers as mathematical basis
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dual {
    pub val: f64,
    pub dual: f64,
}

/// Constructors
impl Dual {
    pub fn num(val: f64, dual: f64) -> Self {
        Self { val, dual }
    }
    pub fn var(val: f64) -> Self {
        Self::num(val, 1.)
    }
    pub fn par(val: f64) -> Self {
        Self::num(val, 0.)
    }
}

/// Utilities
impl Dual {
    /// Apply custom function-derivative pair to a dual number.
    /// May be used to extend the provided set of functions
    /// ```
    /// trait OpsExtended{
    ///     fn powi(self, n: i32) -> Self;
    /// }
    ///
    /// use autodj::Dual;
    ///
    /// impl OpsExtended for Dual {
    ///     fn powi(self, n: i32) -> Self {
    ///         self.custom(
    ///             |x| x.powi(n),//
    ///             |x| x.powi(n - 1) * (n as f64)
    ///         )
    ///     }
    /// }
    ///
    /// let x = Dual::var(std::f64::consts::PI);
    /// let n = 2;
    /// let f = x.powi(n);
    /// ```
    pub fn custom<F, D>(self, func: F, deriv: D) -> Self
    where
        F: Fn(f64) -> f64,
        D: Fn(f64) -> f64,
    {
        Self {
            val: func(self.val),
            dual: self.dual * deriv(self.val),
        }
    }

    fn chain_of(self, val: f64, deriv: f64) -> Self {
        Self {
            val,
            dual: deriv * self.dual,
        }
    }
}

/// basic arithmetic
impl Dual {
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

/// commonly used functions
impl Dual {
    pub fn powf(self, p: f64) -> Self {
        self.custom(
            |x| x.powf(p), //
            |x| x.powf(p - 1.) * p,
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
            |x| x.ln(), //
            |x| 1. / x,
        )
    }

    pub fn abs(self) -> Self {
        self.custom(
            |x| x.abs(), //
            |x| x.signum(),
        )
    }

    pub fn signum(self) -> Self {
        self.custom(
            |x| x.signum(), //
            |_| 0.,
        )
    }
}

impl Add for Dual {
    type Output = Dual;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

impl Sub for Dual {
    type Output = Dual;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub(rhs)
    }
}

impl Mul for Dual {
    type Output = Dual;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(rhs)
    }
}

impl Div for Dual {
    type Output = Dual;

    fn div(self, rhs: Self) -> Self::Output {
        self.div(rhs)
    }
}

impl Neg for Dual {
    type Output = Dual;

    fn neg(self) -> Self::Output {
        self.neg()
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}

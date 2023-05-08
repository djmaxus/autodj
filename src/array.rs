//! [`crate::array::DualNumber`] for a specific number of variables

use crate::common::Common;
use std::{
    fmt::LowerExp,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

pub use crate::fluid::Dual;

/// Data structure specialization
///```
/// use autodj::array::*;
/// let x0 : DualNumber<2> = 1.0.into(); // Parameter
/// let &[x, y] = [2.,3.].into_variables().get();
/// let f = (x - x0) * y;
/// assert_eq!(f.value(), 3.);
/// assert_eq!(f.grad().len(), 2);
/// ```
pub type DualNumber<const N: usize> = Common<Array<N>>;

impl<const N: usize> DualNumber<N> {
    /// Refer to the contained gradient
    #[must_use]
    pub fn grad(&self) -> &[f64; N] {
        &self.dual_borrow().0
    }

    /**
    Differential (value & sum of derivatives)
    ```
    use autodj::array::*;
    let [x, y] = [2.,2.].into_variables().get().to_owned();
    let f = (x - 1.0.into()) * y;
    let differential = f.differential();
    assert_eq!(differential.value(), 2.);
    assert_eq!(differential.deriv(), 3.);
    ```
    */
    #[must_use]
    pub fn differential(&self) -> Common<f64> {
        Common::<f64>::new(self.value().to_owned(), self.grad().iter().sum())
    }
}

/// Array of dual components
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Array<const N: usize>([f64; N]);

/// Keeps independent variables as a whole
pub struct DualVariables<const N: usize> {
    /// Array storage of variables
    variables: [DualNumber<N>; N],
}

impl<const N: usize> DualVariables<N> {
    /// Refer to an array of independent variables with their own unit dual component
    #[must_use]
    pub fn get(&self) -> &[DualNumber<N>; N] {
        &self.variables
    }

    /// Evaluate a function over the variables
    pub fn eval<Out>(&self, func: impl Fn(&[DualNumber<N>; N]) -> Out) -> Out {
        func(self.get())
    }
}

impl<const N: usize, T: Into<[f64; N]>> From<T> for DualVariables<N> {
    fn from(values: T) -> Self {
        let mut variables: [DualNumber<N>; N] = Into::into(values).map(Into::into);
        variables.iter_mut().enumerate().for_each(|(i, x)| {
            x.dual_mut().0[i] = 1f64;
        });
        DualVariables { variables }
    }
}

/// Convenient wrapper for calling [`Into<DualVariables<N>>`]
pub trait IntoVariables<const N: usize> {
    /// Convert to [`DualVariables<N>`]
    fn into_variables(self) -> DualVariables<N>;
}

impl<T, const N: usize> IntoVariables<N> for T
where
    T: Into<[f64; N]>,
{
    fn into_variables(self) -> DualVariables<N> {
        self.into().into()
    }
}

impl<const N: usize> Default for Array<N> {
    fn default() -> Self {
        Array([Default::default(); N])
    }
}

impl<const N: usize> std::fmt::Display for Array<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+{:?}", self.0)
    }
}

impl<const N: usize> std::ops::Mul<f64> for Array<N> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0.map(|x| x * rhs))
    }
}

impl<const N: usize> std::ops::MulAssign<f64> for Array<N> {
    fn mul_assign(&mut self, rhs: f64) {
        self.0.iter_mut().for_each(|x| *x *= rhs);
    }
}

impl<const N: usize> std::ops::Div<f64> for Array<N> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0.map(|x| x / rhs))
    }
}

impl<const N: usize> Neg for Array<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Array(self.0.map(|x| -x))
    }
}

impl<const N: usize> LowerExp for Array<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+[")?;
        for index in 1..=N {
            write!(f, "{:e}", self.0[index - 1])?;
            if index == N {
                break;
            }
            write!(f, ", ")?;
        }
        write!(f, "]")
    }
}

impl<const N: usize> Add<Array<N>> for Array<N> {
    type Output = Array<N>;

    fn add(self, rhs: Self) -> Self::Output {
        binary_operation(&self, &rhs, f64::add_assign)
    }
}

impl<const N: usize> Sub for Array<N> {
    type Output = Array<N>;
    fn sub(self, rhs: Self) -> Self::Output {
        binary_operation(&self, &rhs, f64::sub_assign)
    }
}

/// Common implementation of binary operations
fn binary_operation<const N: usize, Op>(a: &Array<N>, b: &Array<N>, op: Op) -> Array<N>
where
    Op: Fn(&mut f64, f64),
{
    let mut out = a.0;

    out.iter_mut().zip(b.0.iter()).for_each(|(x, &y)| op(x, y));

    Array(out)
}

impl<const N: usize> AddAssign for Array<N> {
    fn add_assign(&mut self, rhs: Self) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(x, &y)| *x += y);
    }
}

impl<const N: usize> SubAssign for Array<N> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(x, &y)| *x -= y);
    }
}

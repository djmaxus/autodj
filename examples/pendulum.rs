use autodj::array::*;
use ergo_traits::*;
use nalgebra::{base::Scalar, vector, ArrayStorage, SMatrix, SVector};
use std::{
    error::Error,
    f64::consts::PI,
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};
type Dual2 = DualNumber<2>;
type V2<T> = SVector<T, 2>;
type M2<T> = SMatrix<T, 2, 2>;

fn u_dot<T>(v: T) -> T {
    v
}

trait RealOps:
    Sub<Output = Self>
    + MulAssign
    + Mul<Output = Self>
    + Sized
    + Clone
    + PartialEq
    + SubAssign
    + Debug
    + From<f64>
    + DivAssign
    + Div<Output = Self>
    + Copy
    + Scalar
    + AddAssign
    + Add<Output = Self>
{
    fn sin(&self) -> Self;
}

impl RealOps for f64 {
    fn sin(&self) -> Self {
        f64::sin(*self)
    }
}

impl RealOps for Dual2 {
    fn sin(&self) -> Self {
        Dual2::sin(self)
    }
}

fn v_dot<T: RealOps>(u: T, kappa: f64) -> T {
    u.sin() * kappa.into_erg()
}

fn calc_x_dot<T: RealOps>(x: V2<T>, kappa: f64) -> V2<T> {
    vector![u_dot(x[1]), v_dot(x[0], kappa)]
}

fn calc_x_dot_numeric<T: RealOps>(x0: V2<T>, x1: V2<T>, dt: f64) -> V2<T> {
    (x1 - x0) / dt.into_erg()
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct Fraction(f64);

impl TryFrom<f64> for Fraction {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value < 0. {
            return Err("value is less than zero");
        }
        if value > 1. {
            return Err("value is greater than one");
        }
        Ok(Fraction(value))
    }
}

#[allow(unused)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
enum OdeScheme {
    EulerExplicit,
    EulerImplicit,
    MidPoint,
    InterMediate(Fraction),
}

impl OdeScheme {
    pub fn approx<T: RealOps>(&self, x: [V2<T>; 2]) -> V2<T> {
        match self {
            Self::EulerExplicit => x[0],
            Self::EulerImplicit => x[1],
            Self::MidPoint => (x[0] + x[1]) * 0.5.into_erg(),
            Self::InterMediate(Fraction(fraction)) => Self::interpolate(x, fraction.to_owned()),
        }
    }

    fn interpolate<T: RealOps>(x: [V2<T>; 2], weight: f64) -> V2<T> {
        x[0] * (1.0 - weight).into_erg() + x[1] * weight.into_erg()
    }
}

fn calc_residual<T: RealOps>(kappa: f64, x: [V2<T>; 2], dt: f64, ode_scheme: OdeScheme) -> V2<T> {
    calc_x_dot_numeric(x[0], x[1], dt) - calc_x_dot(ode_scheme.approx(x), kappa)
}

trait IntoSVector<Input, const N: usize> {
    fn into_s_vector<T: From<Input>>(self) -> SVector<T, N>;
}

impl<const N: usize, InputArray: Into<[Input; N]>, Input> IntoSVector<Input, N> for InputArray {
    fn into_s_vector<T: From<Input>>(self) -> SVector<T, N> {
        let arr = self.into().map(Into::into);
        let arr_storage = ArrayStorage([arr; 1]);
        SVector::<T, N>::from_data(arr_storage)
    }
}

fn newton_iterations<F>(
    calc_residual: F,
    x_approx: V2<f64>,
    num_iterations: usize,
    tolerance: f64,
) -> (V2<f64>, Option<f64>)
where
    F: Fn(V2<Dual2>) -> V2<Dual2>,
{
    let tolerance = tolerance.abs();

    let mut x = x_approx;
    let mut error = Option::None;

    for _ in 0..num_iterations {
        let vars: DualVariables<2> = x.into_variables();
        let x_current = vars.get().into_s_vector::<Dual2>();

        let residual_dual = calc_residual(x_current);

        let residual =
            V2::<f64>::from_iterator(residual_dual.iter().map(|equation| equation.value()));

        error = residual.norm().into_some();

        if error.unwrap() <= tolerance {
            break;
        }

        let jacobian = M2::<f64>::from_row_iterator(
            residual_dual
                .iter()
                .flat_map(|equation| equation.grad().to_owned()),
        );

        let increment = jacobian.qr().solve(&residual).unwrap();

        x -= increment;
    }

    (x, error)
}

fn main() -> Result<(), Box<dyn Error>> {
    let kappa = 1.;
    let x0 = [PI * 0.25, 0.].into_s_vector::<f64>();
    let dt: f64 = 1.0;
    let ode_scheme = OdeScheme::InterMediate(1.0.try_into()?);

    let x0_dual = x0.into_s_vector::<Dual2>();

    let x_approx: V2<f64> = x0 + calc_x_dot(x0, kappa) * dt;

    let calc_residual_problem = |x0, x_approx| calc_residual(kappa, [x0, x_approx], dt, ode_scheme);

    let calc_residual_time_step = |x| calc_residual_problem(x0_dual, x);

    let x1 = newton_iterations(calc_residual_time_step, x_approx, 10, 1e-3);

    dbg!(x1);

    ().into_ok()
}

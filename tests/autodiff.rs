//! Implementation of the examples from [`autodiff`] using [`autodj`]
//!

use autodiff::{F, F1};

#[test]
fn quadratic() {
    let x: f64 = 0.0;

    let autodiff = {
        let f = |x: F1| -> F1 { (x - 1.0f64).pow(2.0) };
        let dfdx = f(F1::var(x));
        dfdx.deriv()
    };

    use autodj::single::*;
    let autodj = {
        fn calculate_quadratic(x: DualNumber) -> DualNumber {
            let shift: DualNumber = 1.0.into();
            (x - shift).powi(2)
        }
        x.into_variable().eval(calculate_quadratic)
    };

    assert_eq!(autodj.deriv(), autodiff);
    println!(
        r#"
----------f(x) = (x - 1)^2
autodiff: df/dx = {} at x = {}
autodj  : f({}) ≈ {}"#,
        autodiff, x, x, autodj,
    );
}

#[test]
fn multi_quadratic() {
    let x: f64 = 0.0;
    let y: f64 = 0.0;

    let autodiff = {
        let f = |x: F1, y: F1| -> F1 { (x - 1.0f64) * (2. * y - 1.0f64) };

        let dfdx = f(F1::var(x), F::cst(y));
        let dfdy = f(F::cst(x), F1::var(y));

        [dfdx.deriv(), dfdy.deriv()]
    };

    use autodj::array::*;
    let autodj = {
        fn calculate_multi_quadratic(&[x, y]: &[DualNumber<2>; 2]) -> DualNumber<2> {
            let shift = 1.0.into();
            (x - shift) * (y * 2.0.into() - shift)
        }
        [x, y].into_variables().eval(calculate_multi_quadratic)
    };

    assert_eq!(autodj.grad(), &autodiff);

    println!(
        r#"
----------f(x,y) = (x - 1) * (2 * y - 1)
autodiff: df/dx = {} and df/dy = {} at x = {}, y = {}
autodj  : f({:?}) ≈ {}"#,
        autodiff[0],
        autodiff[1],
        x,
        y,
        [x, y],
        autodj
    );
}

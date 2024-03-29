//! Implementation of the examples from [`autodiff`] using [`autodj`]
#![allow(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::default_numeric_fallback,
    clippy::indexing_slicing,
    clippy::similar_names,
    clippy::float_cmp
)]

use autodiff::{F, F1};

#[test]
fn quadratic() {
    use autodj::prelude::single::*;

    let x: f64 = 0.0;

    let autodiff = {
        let f = |x: F1| -> F1 { (x - 1.0f64).pow(2.0) };
        let dfdx = f(F1::var(x));
        dfdx.deriv()
    };

    let autodj = {
        fn calculate_quadratic(x: DualF64) -> DualF64 {
            let shift: DualF64 = 1.0.into();
            (x - shift).powf(2.0)
        }
        x.into_variable().map(calculate_quadratic)
    };

    assert_eq!(autodj.dual().to_owned(), autodiff);

    println!(
        r#"
----------f(x) = (x - 1)^2
autodiff: df/dx = {autodiff} at x = {x}
autodj  : f({x}) ≈ {autodj}"#
    );
}

#[test]
fn multi_quadratic() {
    use autodj::prelude::array::*;

    let x: f64 = 0.0;
    let y: f64 = 0.0;

    let autodiff = {
        let f = |x: F1, y: F1| -> F1 { (x - 1.0f64) * (2. * y - 1.0f64) };

        let dfdx = f(F1::var(x), F::cst(y));
        let dfdy = f(F::cst(x), F1::var(y));

        [dfdx.deriv(), dfdy.deriv()]
    };

    let autodj = {
        fn calculate_multi_quadratic([x, y]: [DualNumber<f64, 2>; 2]) -> DualNumber<f64, 2> {
            let shift = 1.0.into();
            (x - shift) * (y * 2.0.into() - shift)
        }
        calculate_multi_quadratic([x, y].into_variables())
    };

    assert_eq!(autodj.dual().as_ref(), &autodiff);

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

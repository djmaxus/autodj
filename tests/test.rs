//! Integration tests for [`autodj`]

mod ideal_gas {
    use autodj::fluid::Dual;

    fn calc_gas_state<D: Dual + From<f64>>([pressure, volume, temperature, moles]: [D; 4]) -> D {
        const UGC: f64 = 8.314;
        pressure * volume - temperature * moles * UGC.into()
    }

    const ATM: f64 = 101325.;
    const GOLDEN: f64 = 1.61803398875;
    const KELVIN: f64 = 273.15;
    const BODY: f64 = KELVIN + 36.6;

    #[test]
    fn moles() {
        use autodj::single::*;

        let pressure = ATM.into();
        let volume = GOLDEN.into();
        let temperature = BODY.into();

        let moles_initial = 1.0;

        let scalar_func = |m| calc_gas_state([pressure, volume, temperature, m]);

        let initial = moles_initial.into_variable().map(scalar_func);

        let (f, df) = initial.decompose();
        // newton iteration
        let moles = moles_initial - f / df;

        let state = moles.into_variable().map(scalar_func);

        println!(
            r#"
Initial guess: r({moles_initial}) = {initial}
Update-------: r({moles}) = {:e}"#,
            state.value()
        );
    }

    #[test]
    fn moles_volume() {
        let pressure = ATM.into();
        let temperature = BODY.into();

        let moles_initial = GOLDEN;
        let volume_initial = 1.0;

        use autodj::array::*;

        let vector_func = |&[moles, volume]: &[DualNumber<f64, 2>; 2]| {
            calc_gas_state([pressure, volume, temperature, moles])
        };

        let initial = vector_func(&[moles_initial, volume_initial].into_variables());

        // Newton-like iteration
        const W: f64 = 1.5;
        const WEIGHTS: [f64; 2] = [W, 1. - W];

        let moles = moles_initial - WEIGHTS[0] * initial.value() / initial.dual().as_ref()[0];
        let volume = volume_initial - WEIGHTS[1] * initial.value() / initial.dual().as_ref()[1];

        let update = vector_func(&[moles, volume].into_variables());

        println!(
            r#"
Initial guess: r({moles_initial}, {volume_initial}) = {initial:e}
Update-------: r({moles}, {volume}) = {:e}"#,
            update.value()
        );
    }
}

mod vector {

    use autodj::{fluid::Dual, vector::*};
    use std::ops::{Add, Mul};
    #[test]
    fn vector_multiple() {
        fn sqps(x: &[DualF64]) -> DualF64 {
            let add: DualF64 = 1.0.into();
            x.iter()
                .map(|x| x.powf(2.0).add_impl(&add))
                .reduce(Add::add)
                .expect("nonzero slice length")
        }
        let x = vec![1., 2., 3.];
        let result = sqps(x.clone().into_variables().as_slice());
        println!("f{x:?} ≈ {result:?}");
        // FIXME: add assertions
    }

    #[test]
    fn sum() {
        let x = vec![1., 2., 3., 4., 5.];

        let reference: f64 = x.iter().sum();
        println!("f({x:?}) = ∑ x_i = {}", reference);

        let result: DualF64 = x
            .clone()
            .into_variables()
            .into_iter()
            .reduce(Add::add)
            .expect("nonzero slice length");
        println!("f({x:?}) = {result:?}",);

        assert_eq!(result.value(), &reference);
    }

    #[test]
    fn product() {
        let x = vec![1., 2., 3., 4., 5.];

        let reference: f64 = x.iter().product();
        println!("f({x:?}) = ∏ x_i = {}", reference);

        let result = x.clone().into_variables().into_iter().reduce(Mul::mul);
        println!("f({x:?}) = {result:?}",);

        assert_eq!(
            result.map(|result| result.value().to_owned()),
            Some(reference)
        );
    }

    #[test]
    fn product_owned() {
        let zero = [1.0, 2.0, 3.0]
            .into_variables()
            .iter()
            .map(|x| x.sub_impl(&1.0.into()))
            .reduce(Mul::mul);

        assert_eq!(zero.map(|result| result.value().to_owned()), Some(0.0));
    }

    #[test]
    fn shifted_partial() {
        fn shifted_product(x: &[DualF64], threshold: f64) -> Option<DualF64> {
            x.iter()
                .filter(|x| x.value() < &threshold)
                .map(|x| x.sub_impl(&1.0.into()))
                .reduce(Mul::mul)
        }

        let values: Vec<f64> = vec![2., 3., 5., 8.];
        let variables = values.clone().into_variables();
        let f = shifted_product(variables.as_slice(), 6.).expect("At least two variables");
        println!("f({:?}) = {:?}", values, f);
        assert_eq!(f.value(), &8.);
        assert_eq!(f.dual().as_ref().len(), variables.len());
        assert_eq!(f.dual().as_ref().last(), Some(&0.));
    }

    #[test]
    fn div() {
        let variables = [1., 2.].into_variables();
        let x = &variables[0];
        let y = &variables[1];

        let (f, df) = (x.div_impl(y)).decompose();
        assert_eq!(df.as_ref(), &[0.5, -0.25]);
        assert_eq!(f, 0.5);
    }
}

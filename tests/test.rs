//! Integration tests for [`autodj`]



mod ideal_gas {
    use autodj::common::*;

    fn calc_gas_state<D: DualComponent>(
        [pressure, volume, temperature, moles]: [DualCommon<D>; 4],
    ) -> DualCommon<D>
    where
        D: DualComponent,
    {
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

        let initial = moles_initial.into_variable().eval(scalar_func);

        // newton iteration
        let moles = moles_initial - initial.value() / initial.deriv();

        let state = moles.into_variable().eval(scalar_func);

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

        let vector_func = |&[moles, volume]: &[DualNumber<2>; 2]| {
            calc_gas_state([pressure, volume, temperature, moles])
        };

        let initial = [moles_initial, volume_initial]
            .into_variables()
            .eval(vector_func);

        // Newton-like iteration
        const W: f64 = 1.5;
        const WEIGTS: [f64; 2] = [W, 1. - W];

        let moles = moles_initial - WEIGTS[0] * initial.value() / initial.grad()[0];
        let volume = volume_initial - WEIGTS[1] * initial.value() / initial.grad()[1];

        let update = [moles, volume].into_variables().eval(vector_func);

        println!(
            r#"
Initial guess: r({moles_initial}, {volume_initial}) = {initial:e}
Update-------: r({moles}, {volume}) = {:e}"#,
            update.value()
        );
    }
}

mod vector {
    use autodj::vector::*;

    #[test]
    fn vector_multiple() {
        fn sqps(x: &[DualNumber]) -> DualNumber {
            let add: DualNumber = 1.0.into();
            x.iter().map(|x| x.powf(2.0) + &add).sum()
        }
        let x = vec![1., 2., 3.];
        let result = x.into_variables().eval(|x| sqps(x.as_slice()));
        println!("f{x:?} ≈ {result:?}");
    }

    #[test]
    fn sum() {
        let x = vec![1., 2., 3., 4., 5.];

        let reference: f64 = x.iter().sum();
        println!("f({x:?}) = ∑ x_i = {}", reference);

        let result: DualNumber = x.into_variables().get().iter().sum();
        println!("f({x:?}) = {result:?}",);

        assert_eq!(result.value(), reference);
    }

    #[test]
    fn product() {
        let x = vec![1., 2., 3., 4., 5.];

        let reference: f64 = x.iter().product();
        println!("f({x:?}) = ∏ x_i = {}", reference);

        let result: DualNumber = x.into_variables().get().iter().product();
        println!("f({x:?}) = {result:?}",);

        assert_eq!(result.value(), reference);
    }

    #[test]
    fn product_owned() {
        let zero: DualNumber = [1.0, 2.0, 3.0]
            .into_variables()
            .get()
            .iter()
            .map(|x| x - &1.0.into())
            .product();
        assert_eq!(zero.value(), 0.0);
    }

    #[test]
    fn shifted_partial() {
        fn shifted_product(x: &[DualNumber], threshold: f64) -> DualNumber {
            x.iter()
                .filter(|x| x.value() < threshold)
                .map(|x| x - &1.0.into())
                .product()
        }

        let values: Vec<f64> = vec![2, 3, 5, 8].iter().map(|&x| x as f64).collect();

        let variables = values.into_variables();
        let f = shifted_product(variables.get(), 6.);
        println!("f({:?}) = {:?}", values, f);
        assert_eq!(f.value(), 8.);
        assert_eq!(f.grad().len(), variables.get().len() - 1);
    }

    #[test]
    fn div() {
        let variables = [1., 2.].into_variables();
        let slice = variables.get().get(0..2).unwrap();

        let x = slice.first().unwrap();
        let y = slice.last().unwrap();

        let f = x / y;

        assert_eq!(f.grad(), &[1. / 2., -1. / 4.]);
        assert_eq!(f.value(), 0.5);
    }
}

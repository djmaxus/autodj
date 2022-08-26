use std::ops::{Mul, Sub};

use autodj::multi::*;

fn calc_ideal_gas_state<T>(&pressure: &T, &volume: &T, &temperature: &T, &moles: &T) -> T
where
    T: Sub<Output = T> + Mul<Output = T> + Copy,
{
    pressure * volume - moles * temperature
}

#[test]
#[ignore]
// #[should_panic]
pub fn example_driven_dev() {
    let [pressure, volume, temperature, moles] = [1e5, 1.618, 300., 1.];

    let [temperature, moles] = [temperature, moles].pars();

    let calculator = |&[p, v]: &DualVars<2>| {
        calc_ideal_gas_state(
            &p, //
            &v,
            &temperature,
            &moles,
        )
    };

    let state = [pressure, volume].eval(&calculator);

    println!("{:?}", state);
}

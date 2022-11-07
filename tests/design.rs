use autodj::multi::*;
use std::ops::{Mul, Sub};
fn calc_ideal_gas_state<T>(&pressure: &T, &volume: &T, &temperature: &T, &moles: &T) -> T
where
    T: Sub<Output = T> + Mul<Output = T> + Copy,
{
    pressure * volume - moles * temperature
}

#[test]
#[ignore]
pub fn ideal_gas_multi() {
    let [pressure, volume, temperature, moles] = [1e5, 1.618, 300., 1.];

    let [temperature, moles] = [temperature, moles].pars();

    let calculator = |&[p, v]: &DualVars<2>| calc_ideal_gas_state(&p, &v, &temperature, &moles);

    let state = [pressure, volume].eval(&calculator);

    println!("{:?}", state);
}

#[test]
#[ignore]
pub fn sync_multi_n_single() {
    let x = [2., 3.].vars();
    let p = [5.0, 7.0].pars();

    fn calc_f(x: &DualVars<2>, p: &[DualNumber<2>; 2]) -> Dual2D {
        p[0] * x[0] + x[1] * p[1]
    }

    let f = calc_f(&x, &p);
    println!("gradient = {:?}, df/dy= {}", f.grad(), f.grad()[1],);

    let x = Dual2D::new(2., &[3., 5.]);
    let x = [x.val(), 11.];

    let closure = |args: &DualVars<2>| args[0] + args[1];

    let f = x.eval(&closure);

    println!("gradient = {:?}, df/dx= {}", f.grad(), f.grad()[0],);
}

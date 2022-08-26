//! # `ideal_gas`
//!
//! Run this example with:
//! ```sh
//! cargo run --example ideal_gas
//! ```
//!
//! ## Setup
//!
//! Find pressure that satisfies the ideal gas equation
//!
//! $PV = nT$
//!
//! where
//!
//! | Notation | Meaning         | value               |
//! |---------:|:----------------|:--------------------|
//! |      $P$ | pressure        | 1.0 (initial guess) |
//! |      $V$ | volume          | 1.618               |
//! |      $n$ | number of moles | 1.0                 |
//! |      $T$ | temperature     | 300.0               |
//!
//! ## Solution
//!
//! In this example, we use Newton method to solve the equation.
//!

use autodj::single::*;

fn main() {
    let [pressure, volume, temperature, quantity]: [f64; 4] = [1., 1.618, 300., 1.];

    let calc_residual_dual = |x: &DualNumber| {
        calc_ideal_gas_generic(
            *x, //
            volume.par(),
            temperature.par(),
            quantity.par(),
        )
    };

    let residual_generic = pressure.eval(&calc_residual_dual);

    print_state_linearization(
        residual_generic.val(), //
        residual_generic.deriv(),
        &pressure,
    );

    let pressure_newtoned = newton_iterations(
        calc_residual_dual, //
        pressure,
        1e-3,
        10,
    );

    match pressure_newtoned {
        Ok(pressure_refined) => {
            println!("{pressure} refined to {pressure_refined} using Newton method")
        }
        Err(ConvergenceError(err)) => {
            println!("Not converged Newton iterations:");
            match err {
                Some(err) => println!("----with an error of {}", err),
                None => println!("----function was not evaluated"),
            }
        }
    }
}

fn print_state_linearization(value: &f64, deriv: &f64, origin: &f64) {
    println!("Linearization: {value} + {deriv} * (pressure - {origin})");
}

fn _calc_ideal_gas(pressure: f64, volume: f64, temperature: f64, quantity: f64) -> f64 {
    pressure * volume - quantity * temperature
}

use std::ops::{Mul, Sub};
fn calc_ideal_gas_generic<T>(pressure: T, volume: T, temperature: T, quantity: T) -> T
where
    T: Mul<Output = T> + Sub<Output = T>,
{
    pressure * volume - quantity * temperature
}

fn _calc_ideal_gas_deriv(volume: f64) -> f64 {
    volume
}

fn newton_iterations<Resid>(
    func: Resid,
    initial: f64,
    tolerance: f64,
    max_iter: u8,
) -> Result<f64, ConvergenceError>
where
    Resid: DualFunction,
{
    let mut result = initial;

    let mut calc = None;

    for _ in 0..=max_iter {
        calc = Some(result.eval(&func));

        let error = (calc.unwrap().val() - tolerance).abs();

        if error <= tolerance {
            return Ok(result);
        }

        let delta = -calc.unwrap().val() / calc.unwrap().deriv();

        result += delta;
    }

    Err(ConvergenceError(calc.map_or(None, |x| Some(*x.val()))))
}
struct ConvergenceError(Option<f64>);

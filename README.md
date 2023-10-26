# Automatic Differentiation Library

<!-- FIXME: actualize README -->

[![crates.io](https://img.shields.io/crates/v/autodj.svg)](https://crates.io/crates/autodj)
[![docs](https://docs.rs/autodj/badge.svg)](https://docs.rs/autodj/)
![build](https://github.com/djmaxus/autodj/actions/workflows/rust.yml/badge.svg?branch=master)
![rust-clippy analyze](https://github.com/djmaxus/autodj/actions/workflows/rust-clippy.yml/badge.svg?branch=master)

**AUTO**matic **D**erivatives & **J**acobians
by [djmaxus](https://djmaxus.github.io/) and [you](https://github.com/djmaxus/autodj/issues)

- [Functionality](#functionality)
  - [Single variables](#single-variables)
  - [Multiple variables](#multiple-variables)
    - [Static number of variables](#static-number-of-variables)
    - [Dynamic number of variables](#dynamic-number-of-variables)
  - [Generic dual numbers](#generic-dual-numbers)
- [Motivation](#motivation)
- [Project goals](#project-goals)
- [Anticipated features](#anticipated-features)
- [Comparison with `autodiff`](#comparison-with-autodiff)

## Functionality

### Single variables

```rust
use autodj::prelude::single::*;

let x : DualF64 = 2.0.into_variable();

// Arithmetic operations are required by trait bounds
let _f = x * x + 1.0.into();

// Arithmetic rules itself are defined in `Dual` trait
// on borrowed values for extendability
let f = (x*x).add_impl(&1.0.into());

// Dual can be decomposed into a value-derivative pair
assert_eq!(f.decompose(), (5.0, 4.0));

// fmt::Display resembles Taylor expansion
assert_eq!(format!("{f}"), "5+4∆");
```

### Multiple variables

Multivariate differentiation is based on **multiple dual components**.
Such an approach requires **no repetitive and "backward" differentiations**.
Each partial derivative is tracked separately from the start,
and no repetitive calculations are made.

For built-in multivariate specializations,
independent variables can be created consistently using `.into_variables()` method.

#### Static number of variables

```rust
use autodj::prelude::array::*;

// consistent set of independent variables
let [x, y] : [DualNumber<f64,2>; 2] = [2.0, 3.0].into_variables();

let f = x * (y - 1.0.into());

assert_eq!(f.value()        , & 4.);
assert_eq!(f.dual().as_ref(), &[2., 2.]);
assert_eq!(format!("{f}")   , "4+[2.0, 2.0]∆");
```

#### Dynamic number of variables

```rust
use autodj::prelude::vector::*;
use std::ops::Add;

let x = vec![1., 2., 3., 4., 5.].into_variables();

let f : DualF64 = x.iter()
                   .map(|x : &DualF64| x.mul_impl(&2.0.into()))
                   .reduce(Add::add)
                   .unwrap();

assert_eq!(f.value(), &30.);

f.dual()
 .as_ref()
 .iter()
 .for_each(|deriv| assert_eq!(deriv, &2.0) );
```

### Generic dual numbers

```rust
// A trait with all the behavior defined
use autodj::fluid::Dual;
// A generic data structure which implements Dual
use autodj::solid::DualNumber;
```

## Motivation

I do both academic & business R&D in the area of computational mathematics.
As well as many of us, I've written a whole bunch of sophisticated Jacobians _by hand_.

One day, I learned about automatic differentiation based on dual numbers.
Almost the same day, I learned about Rust as well :crab:

Then, I decided to:

- Make it automatic and reliable as much as possible
- Use modern and convenient ecosystem of Rust development

## Project goals

- Develop open-source automatic differentiation library for both _academic_ and _commercial_ computational mathematicians
- Gain experience of Rust programming

## Anticipated features

You are very welcome to introduce [issues](https://github.com/djmaxus/autodj/issues/new/choose)
to promote most wanted features or to report a bug.

- [x] Generic implementation of dual numbers
- Number of variables to differentiate
  - [x] single
  - multiple
    - [x] static
    - [x] dynamic
    - [x] sparse
  - [ ] Jacobians (efficient layouts in memory to make matrices right away)
- [x] Named variables (UUID-based)
- [ ] Calculation tracking (partial derivatives of intermediate values)
- Third-party crates support (as features)
  - [x] `num-traits`
  - [x] linear algebra crates (`nalgebra` etc.)
- Advanced features
  - [x] Arbitrary number types beside `f64`
  - [ ] Inter-operability of different dual types (e.g., single and multiple dynamic)
  - [ ] Numerical verification (or replacement) of derivatives (by definition)
  - [ ] Macro for automatic extensions of regular (i.e. non-dual) functions
  - [ ] Optional calculation of derivatives
    - [ ] Backward differentiation probably
    - [ ] Iterator implementation as possible approach to lazy evaluation

## Comparison with [`autodiff`](https://crates.io/crates/autodiff)

As far as I noticed, `autodj` currently has the following differences

- Multiple variables out of the box
- `fmt::Display` for statically-known number of variables
- Left-to-right flow of many operations such as `.into-variables()`, `.eval()`, etc.
- Number type is restricted to `f64`
- No utilization of `num` and `nalgebra` crates

Some differences are planned to be eliminated as noted in the [roadmap](#anticipated-features).

Within this crate, you may study & launch test target `/tests/autodiff.rs`
to follow some differences.

```shell
cargo test --test autodiff -- --show-output
```

# Automatic Differentiation Library

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
use autodj::single::*;

let x : DualNumber = 2.0.into_variable();

// values can be borrowed for arithmetic operations
let f = x * x + &1.0.into();

assert_eq!(f.value(), &5.0);
assert_eq!(f.deriv(),  4.0);

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
use autodj::array::*;

// consistent set of independent variables
let vars : DualVariables<2> = [2.0, 3.0].into_variables();
let [x, y] = vars.get().to_owned();

let f = x * (y - 1.0.into());

assert_eq!(f.value(), & 4.);
assert_eq!(f.grad() , &[2., 2.]);
assert_eq!(format!("{f}"), "4+[2.0, 2.0]∆");
  ```

#### Dynamic number of variables

```rust
use autodj::vector::*;

let x : DualVariables = vec![1., 2., 3., 4., 5.].into_variables();

let f : DualNumber = x.get()
                      .iter()
                      .map(|x : &DualNumber| x * &2.0.into())
                      .sum();

assert_eq!(f.value(), &30.);

f.grad()
 .iter()
 .for_each(|deriv| assert_eq!(deriv, &2.0) );
```

### Generic dual numbers

```rust
// can be specialized for your needs
use autodj::common::Common;
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
    - [ ] sparse
  - [ ] Jacobians for efficient layouts in memory
- [ ] Named variables (UUID-based)
- [ ] Calculation tracking (partial derivatives of intermediate values)
- Third-party crates support (as features)
  - [ ] `num`
  - [ ] linear algebra crates (`nalgebra` etc.)
- Advanced features
  - [ ] Arbitrary number types beside `f64`
  - [ ] Inter-operability of different dual types (e.g., single and multiple dynamic)
  - [ ] Numerical verification (or replacement) of derivatives (by definition)
  - [ ] Macro for automatic extensions of regular (i.e. non-dual) functions
  - [ ] Optional calculation of derivatives
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

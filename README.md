# Automatic Differentiation Library

[![crates.io](https://img.shields.io/crates/v/autodj.svg)](https://crates.io/crates/autodj)
[![docs](https://docs.rs/autodj/badge.svg)](https://docs.rs/autodj/)
[![build](https://github.com/djmaxus/autodj/actions/workflows/rust.yml/badge.svg)](https://github.com/djmaxus/autodj/)
[![rust-clippy analyze](https://github.com/djmaxus/autodj/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/djmaxus/autodj/actions/workflows/rust-clippy.yml)

**AUTO**matic **D**erivatives & **J**acobians \
by [djmaxus](https://djmaxus.github.io/) and [you](https://github.com/djmaxus/autodj/issues)

- pre-alpha
- play-ready

## Contents

- [Automatic Differentiation Library](#automatic-differentiation-library)
  - [Contents](#contents)
  - [Motivation](#motivation)
  - [Project goals](#project-goals)
  - [Anticipated features](#anticipated-features)

## Motivation

For the living (and for my heart),
I do research & development in the area of computational mathematics
and wrote a whole bunch of sophisticated Jacobians _by hand_.

One day, I learned about automatic differentiation based on dual numbers.
Almost the same day, I learned about Rust as well :crab:

- No more devastating hand-written derivatives!
- No more unsafe code!

## Project goals

- Develop open-source automatic differentiation library for both _academic_ and _commercial_ computational mathematitians
- Gain experience of Rust programming

## Anticipated features

- [x] Basic dual arithmetics as standalone feature
- Number of variables
  - [ ] single
  - multiple
    - [ ] static
    - [ ] dynamic
- [ ] Calculation tracking (partial derivatives of intermediate values)
- Third-party crates support (as features)
  - [ ] `num`
  - [ ] linear algebra crates
- Advanced features

  - [ ] Inter-operability of different dual types (e.g., single and multiple dynamic)
  - [ ] Arbitrary type of dual number components
  - [ ] Rust-alike safety of interfaces

    e.g., `Fn(...) -> Result<_,_>` for binary operations and UUID-based tracking of variables

  - [ ] Numerical verification (or replacement) of derivatives (by definition)
  - [ ] Macro for automatic extensions of regular (i.e. non-dual) functions
  - [ ] no `std`

You are very welcome to introduce [issues](https://github.com/djmaxus/autodj/issues) to promote mowt wanted features or to report a bug.

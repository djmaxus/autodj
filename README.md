by [djmaxus](https://djmaxus.github.io/)

[![crates.io](https://img.shields.io/crates/v/autodj.svg)](https://crates.io/crates/autodj)
[![docs](https://docs.rs/autodj/badge.svg)]()
[![build](https://github.com/djmaxus/autodj/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/djmaxus/autodj/)

# Automatic Differentiation Library

**AUTO**matic **D**erivatives & **J**acobians

## Contents
- [Automatic Differentiation Library](#automatic-differentiation-library)
  - [Contents](#contents)
  - [About](#about)
    - [No more devastating hand-written derivatives!](#no-more-devastating-hand-written-derivatives)
    - [No more unsafe code!](#no-more-unsafe-code)
  - [Project goals](#project-goals)
  - [Roadmap (sort of)](#roadmap-sort-of)

## About

For the living (and for my heart),
I do research & development in the area of computational mathematics
and wrote a whole bunch of sophisticated Jacobians *by hand*.

One day, I learned about automatic differentiation based on dual numbers.
Almost the same day, I learned about Rust as well :crab:

### No more devastating hand-written derivatives!

### No more unsafe code!

## Project goals

* Develop open-source automatic differentiation library
  for both *academic* and *commercial* computational mathematitians
* Gain experience of Rust programming

## Roadmap (sort of)

- [x] Basis of dual math arithmetics
- [ ] Interface prototype
- [ ] Single argument differentiation
- [ ] Documentation / tests
- [ ] Crate tuning (privacy, automation, versioning)
- [ ] Static dual vectors
- [ ] Extension examples for not implemented functions
- [ ] Dynamic dual vectors
- [ ] `num` / `num-traits` support
- [ ] support linalg libraries
- [ ] arbitrary numberic type of dual number components
- [ ] pro-level implementation (with some `#[]` blocks over existing functions)

[github]: https://github.com/djmaxus/autodj
[crates]: https://crates.io/crates/autodj

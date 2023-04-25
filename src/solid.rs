//! Generic data structure implementing [`Dual`]

use crate::fluid::{Grad, Value};

/// Default generic [`Dual`] implementor: a struct with two fields
#[derive(Debug, PartialEq, PartialOrd, Default, Hash)]
pub struct DualNumber<N, D>
where
    N: Value,
    D: Grad<N>,
{
    /// real value
    real: N,
    /// derivatives
    dual: D,
}

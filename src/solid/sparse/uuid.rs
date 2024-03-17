//! [`crate::sparse::DualNumber`] specialization with [`::uuid::Uuid`] as keys to distinguish different variables
#![cfg(feature = "uuid")]

use crate::{
    fluid::{Dual, Value},
    solid::sparse::Grad,
};
use ::uuid::Uuid;
use std::collections::HashMap;

/// Sparse dual number based on [`uuid::Uuid`] keys
pub type DualNumber<V> = crate::solid::sparse::DualNumber<V, Uuid>;

/// Auto-implemented trait for creating independent variables with sparse gradient
pub trait IntoVariable: Value {
    /// Create sparse dual number from some [`crate::fluid::Value`] implementor
    fn into_variable(self) -> DualNumber<Self> {
        let grad_id = Uuid::new_v4();
        let grad_value = Self::one();
        let grad = [(grad_id, grad_value)].into_iter().collect::<HashMap<_, _>>();
        DualNumber::<Self>::new(self, Grad(grad))
    }
}

impl<V: Value> IntoVariable for V {}

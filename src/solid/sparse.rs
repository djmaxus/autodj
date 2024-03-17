//! [`crate::sparse::DualNumber`] based on [`HashMap`] for sparse dual components
#![cfg(feature = "sparse")]

use crate::solid::Value;
use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Add, MulAssign},
};

/// Merge two maps. For common keys, merge values using given binary operation
fn merge_assign<K, V, F>(map1: &mut HashMap<K, V>, map2: &HashMap<K, V>, op: F)
where
    K: Eq + Hash + Clone,
    V: Clone,
    F: Fn(V, V) -> V,
{
    for (key, value) in map2 {
        let _unused = map1
            .entry(key.clone())
            .and_modify(|existing_value| {
                *existing_value = op(existing_value.clone(), value.clone());
            })
            .or_insert(value.clone());
    }
}

/// A bound for keys to use with [`Grad`]
pub trait GradKey: Clone + Eq + Hash {}
impl<T: Clone + Eq + Hash> GradKey for T {}

/// Sparse gradient for dual numbers
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Grad<Key: GradKey, V: Value>(HashMap<Key, V>);

impl<Key: GradKey, V: Value> num_traits::Zero for Grad<Key, V> {
    fn zero() -> Self {
        Self(HashMap::new())
    }

    fn is_zero(&self) -> bool {
        self.0.is_empty() || self.0.values().all(num_traits::Zero::is_zero)
    }
}

impl<Key: GradKey, V: Value> Add for Grad<Key, V> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = self;
        out += rhs;
        out
    }
}

impl<Key: GradKey, V: Value> std::ops::Mul<V> for Grad<Key, V> {
    type Output = Self;

    fn mul(self, rhs: V) -> Self::Output {
        let mut out = self;
        out.mul_assign(rhs);
        out
    }
}

impl<Key: GradKey, V: Value> MulAssign<V> for Grad<Key, V> {
    fn mul_assign(&mut self, rhs: V) {
        self.0.values_mut().for_each(|v| v.mul_assign(rhs));
    }
}

impl<Key: GradKey, V: Value> std::ops::Neg for Grad<Key, V> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut out = self;
        out.0.values_mut().for_each(|v| *v = v.neg());
        out
    }
}

impl<Key: GradKey, V: Value> std::ops::AddAssign for Grad<Key, V> {
    fn add_assign(&mut self, rhs: Self) {
        merge_assign(&mut self.0, &rhs.0, Add::add);
    }
}

impl<Key: GradKey, V: Value> crate::fluid::Grad<V> for Grad<Key, V> {}

impl<Key: GradKey, V: Value> AsRef<HashMap<Key, V>> for Grad<Key, V> {
    fn as_ref(&self) -> &HashMap<Key, V> {
        &self.0
    }
}

/// For sparse gradients
pub type DualNumber<V, Key> = crate::solid::DualNumber<V, Grad<Key, V>>;

pub mod uuid;

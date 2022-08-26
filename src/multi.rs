#[derive(Clone, Copy, Debug)]
pub struct DualNumber<const N: usize> {
    val: f64,
    dual: DualGradInner<N>,
}
pub type Dual2D = DualNumber<2>;
pub type Dual3D = DualNumber<3>;

pub type DualGrad<const N: usize> = [f64; N];
pub type DualVars<const N: usize> = [DualNumber<N>; N];

impl<const N: usize> DualNumber<N> {
    pub fn val(&self) -> f64 {
        self.val
    }
    pub fn grad(&self) -> &DualGrad<N> {
        &self.dual.0
    }
    fn new(val: f64, &dual: &DualGrad<N>) -> Self {
        Self::assert_size();
        let dual = DualGradInner(dual);
        Self { val, dual }
    }
}

impl<const NV: usize> DualOps<NV> for DualVars<NV> {
    fn eval<DF>(&self, func: &DF) -> DualNumber<NV>
    where
        DF: DualFunction<NV>,
    {
        func(self)
    }
}

pub trait DualOps<const NV: usize> {
    fn eval<DF>(&self, func: &DF) -> DualNumber<NV>
    where
        DF: DualFunction<NV>;
}

pub trait Dualize<const NI: usize> {
    fn vars(&self) -> DualVars<NI>;
    fn pars<const NV: usize>(&self) -> [DualNumber<NV>; NI];
    fn eval<DF>(&self, func: &DF) -> DualNumber<NI>
    where
        DF: DualFunction<NI>,
    {
        func(&self.vars())
    }
}

impl<const N: usize> Add for DualGradInner<N> {
    type Output = DualGradInner<N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = self.0;
        (0..N).for_each(|index| {
            out[index] += rhs.0[index];
        });
        Self(out)
    }
}

impl<const N: usize> Sub for DualGradInner<N> {
    type Output = DualGradInner<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = self.0;
        (0..N).for_each(|index| {
            out[index] -= rhs.0[index];
        });
        Self(out)
    }
}

impl<const N: usize> Mul<f64> for DualGradInner<N> {
    type Output = DualGradInner<N>;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut out = self.0;
        out.iter_mut().for_each(|elem| {
            *elem *= rhs;
        });
        Self(out)
    }
}

impl<const N: usize> Div<f64> for DualGradInner<N> {
    type Output = DualGradInner<N>;
    fn div(self, rhs: f64) -> Self::Output {
        let mut out = self.0;
        out.iter_mut().for_each(|elem| {
            *elem /= rhs;
        });
        Self(out)
    }
}

impl<const N: usize> Neg for DualGradInner<N> {
    type Output = DualGradInner<N>;
    fn neg(self) -> Self::Output {
        let mut out = self.0;
        out.iter_mut().for_each(|elem| {
            *elem = -(*elem);
        });
        Self(out)
    }
}

impl<const N: usize> Add for DualNumber<N> {
    type Output = DualNumber<N>;

    fn add(self, rhs: Self) -> Self::Output {
        let val = self.val + rhs.val;
        let dual = self.dual + rhs.dual;
        Self { val, dual }
    }
}

impl<const N: usize> Sub for DualNumber<N> {
    type Output = DualNumber<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let val = self.val - rhs.val;
        let dual = self.dual - rhs.dual;
        Self { val, dual }
    }
}

impl<const N: usize> Mul for DualNumber<N> {
    type Output = DualNumber<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        let val = self.val * rhs.val;
        let dual = self.dual * rhs.val + rhs.dual * self.val;
        Self { val, dual }
    }
}

impl<const N: usize> Div for DualNumber<N> {
    type Output = DualNumber<N>;

    fn div(self, rhs: Self) -> Self::Output {
        let val = self.val / rhs.val;
        let dual = (self.dual * rhs.val - rhs.dual * self.val) / (rhs.val * rhs.val);
        Self { val, dual }
    }
}

impl<const N: usize> Neg for DualNumber<N> {
    type Output = DualNumber<N>;

    fn neg(self) -> Self::Output {
        Self {
            val: -self.val,
            dual: -self.dual,
        }
    }
}

pub trait DualFunction<const NV: usize>: Fn(&DualVars<NV>) -> DualNumber<NV> {}
impl<const NV: usize, F> DualFunction<NV> for F where F: Fn(&DualVars<NV>) -> DualNumber<NV> {}

impl<const NI: usize> Dualize<NI> for [f64; NI] {
    fn vars(&self) -> DualVars<NI> {
        let mut out = [DualNumber::zero(); NI];

        (0..NI).for_each(|index| {
            out[index].val = self[index];
            out[index].dual.0[index] = 1.;
        });

        out
    }

    fn pars<const NV: usize>(&self) -> [DualNumber<NV>; NI] {
        let mut out = [DualNumber::<NV>::zero(); NI];
        (0..NI).for_each(|index| out[index].val = self[index]);
        out
    }
}

impl<const N: usize> DualNumber<N> {
    fn zero() -> Self {
        Self::new(0.0, &[0.; N])
    }

    pub const SIZE_MIN: usize = 2;
    pub const SIZE_MAX: usize = 64;
    const fn assert_size() {
        assert!(N >= Self::SIZE_MIN);
        assert!(N <= Self::SIZE_MAX);
    }
}

impl<const N: usize> Debug for DualGradInner<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn multi_size_0() {
        DualNumber::<0>::zero();
    }

    #[test]
    #[should_panic]
    fn multi_size_1() {
        DualNumber::<1>::zero();
    }

    #[test]
    #[should_panic]
    fn multi_size_min() {
        const SIZE_UNDER: usize = Dual2D::SIZE_MIN - 1;
        DualNumber::<SIZE_UNDER>::zero();
    }

    #[test]
    fn multi_size_inter() {
        Dual2D::zero();
        Dual3D::zero();
    }

    #[test]
    #[should_panic]
    fn multi_size_max() {
        const SIZE_OVER: usize = Dual2D::SIZE_MAX + 1;
        DualNumber::<SIZE_OVER>::zero();
    }
}

use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Clone, Copy)]
struct DualGradInner<const N: usize>(DualGrad<N>);

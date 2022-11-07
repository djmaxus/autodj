pub trait FnXY<X, Y>: Fn(X) -> Y {}
impl<F, X, Y> FnXY<X, Y> for F where F: Fn(X) -> Y {}

pub trait FnBorrowXY<'a, X: 'a, Y>: FnXY<&'a X, Y> {}
impl<'a, F, X: 'a, Y> FnBorrowXY<'a, X, Y> for F where F: FnXY<&'a X, Y> {}

pub trait Fn64: FnXY<f64, f64> {}
impl<F> Fn64 for F where F: FnXY<f64, f64> {}

//! Nightly experiment on generic constant expressions

pub const fn max(a: usize, b: usize) -> usize {
    if a > b {
        return a;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::max;
    trait BinaryOperation<X> {
        type Output;
        fn operation(self, rhs: X) -> Self::Output;
    }

    #[derive(Debug)]
    struct Array<const N: usize>;

    impl<const A: usize, const B: usize> BinaryOperation<Array<B>> for Array<A>
    where
        [(); max(A, B)]:,
    {
        type Output = Array<{ max(A, B) }>;

        fn operation(self, _rhs: Array<B>) -> Self::Output {
            Array
        }
    }

    #[test]
    fn test() {
        let a = Array::<0>;
        print_type_of(&a);

        let b = Array::<2>;
        print_type_of(&b);

        let c = a.operation(b);
        print_type_of(&c);

        let d = Array::<1>;
        print_type_of(&d);
        print_type_of(&c.operation(d));

        fn print_type_of<T>(_: &T) {
            println!("{}", std::any::type_name::<T>())
        }
    }
}

use bevy_utils::all_tuples;

trait Apply<'a, O> {
    type Output;
    fn applicative(self) -> Self::Output; // app is short for applicative
}

macro_rules! impl_apply {
    () => {
        impl<'a, F, O> Apply<'a, (O,)> for F
          where  F: Fn() -> O + 'a,
        {
            type Output =Box<F>;
            fn applicative(self) -> Self::Output {
                Box::new(self)
            }
        }
    };
    ($(($P:ident, $p:ident)),*) => {
        impl<'a, F, O, $($P),*> Apply<'a, (O, $($P),*)> for F
        where
            F: Fn($($P),*) -> O + 'a,
        {
            type Output = Box<dyn Fn(($($P),*,))->O + 'a>;
            fn applicative(self) -> Self::Output {
                Box::new(move |($($p),*,):($($P),*,)| self($($p),*))
            }
        }
    };
}

all_tuples!(impl_apply, 0, 8, P, p_);

#[macro_export]
macro_rules! apply {
    ($e: expr) => {
        $e.applicative()
    };
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_closure() {}

    #[test]
    fn test_apply_macro() {
        let multiply = |a, b| a * b;
        let multiply_fn = multiply.applicative();
        assert_eq!(multiply_fn((3, 4)), 12);
        assert_eq!(multiply_fn((3, 4)), 12);
    }
}

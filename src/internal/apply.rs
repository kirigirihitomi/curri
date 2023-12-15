use bevy_utils::all_tuples;

trait Apply<'a, O> {
    type Output;
    fn app(self) -> Self::Output; // app is short for applicative
}

macro_rules! impl_apply {
    () => {
        impl<'a, F, O> Apply<'a, (O,)> for F
          where  F: Fn() -> O + 'a,
        {
            type Output = F;
            fn app(self) -> Self::Output {
                self
            }
        }
    };
    ($(($P:ident, $p:ident)),*) => {
        impl<'a, F, O, $($P),*> Apply<'a, (O, $($P),*)> for F
        where
            F: Fn($($P),*) -> O + 'a,
        {
            type Output = Box<dyn Fn(($($P),*,))->O + 'a>;
            fn app(self) -> Self::Output {
                Box::new(move |($($p),*,):($($P),*,)| self($($p),*))
            }
        }
    };
}

all_tuples!(impl_apply, 0, 8, P, p_);
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_fn() {
        let add = || 3;
        let add_fn = add.app();
        assert_eq!(add_fn(), 3);
        assert_eq!(add_fn(), 3);
    }

    #[test]
    fn test_apply_closure() {
        let multiply = |a, b| a * b;
        let multiply_fn = multiply.app();
        assert_eq!(multiply_fn((3, 4)), 12);
        assert_eq!(multiply_fn((3, 4)), 12);
    }
}

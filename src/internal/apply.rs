use bevy_utils::all_tuples;

trait Apply<O> {
    type Output;
    fn applicative(self) -> Self::Output; // app is short for applicative
}

macro_rules! impl_apply {
    () => {
        impl<F, O> Apply<(O,)> for F
          where  F: Fn() -> O + 'static,
        {
            type Output =Box<F>;
            fn applicative(self) -> Self::Output {
                Box::new(self)
            }
        }
    };
    ($(($P:ident, $p:ident)),*) => {
        impl<F, O, $($P),*> Apply<(O, $($P),*)> for F
        where
            F: Fn($($P),*) -> O + 'static,
        {
            type Output = Box<dyn Fn(($($P),*,))->O>;
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
    fn test_apply_fn() {
        let add = || 3;
        let add_fn = add.applicative();
        assert_eq!(add_fn(), 3);
        assert_eq!(add_fn(), 3);
    }

    #[test]
    fn test_apply_closure() {
        let multiply = |a, b| a * b;
        let multiply_fn = multiply.applicative();
        assert_eq!(multiply_fn((3, 4)), 12);
        assert_eq!(multiply_fn((3, 4)), 12);
    }

    #[test]
    fn test_apply_macro() {
        let add = || 3;
        let result = apply!(add);
        assert_eq!(result(), 3);
    }
}

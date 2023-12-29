use bevy_utils::all_tuples;

pub trait Curry<'a, O> {
    type Output;
    fn curry(self) -> Self::Output;
}

macro_rules! impl_box_fn_ident {
    () => {Box<dyn Fn() -> O + 'a>};
    ($P0:ident) => {Box<dyn Fn($P0) -> O+ 'a>};
    ($P0:ident, $($P:ident),*) =>{Box<dyn FnOnce($P0) -> impl_box_fn_ident!($($P),*) +'a>};
}

macro_rules! impl_box_fn {
    ($self:ident, (), (), ()) => {Box::new($self)};
    ($self:ident, ($P0:ident), ($p0:ident), ($($p:ident),*)) => {Box::new(move |$p0| $self($($p),*))};
    ($self:ident, ($P0:ident, $($P:ident),*) ,($p0:ident, $($p:ident),*), ($($p_:ident),*)) => {Box::new(move |$p0| impl_box_fn!($self, ($($P),*), ($($p),*), ($($p_),*)))};
}

macro_rules! impl_curry {
    () => {
        impl<'a, F, O,> Curry<'a, (O,)> for F
        where
            F: Fn() -> O + 'a,
        {
            type Output = impl_box_fn_ident!();
            fn curry(self) -> Self::Output {
                impl_box_fn!(self, (), (), ())
            }
        }
    };
    (($LP:ident, $lp:ident)) => {
        impl<'a, F, O, LP> Curry<'a, (O, LP)> for F
        where
            F: Fn(LP) -> O + 'a,
        {
            type Output = impl_box_fn_ident!(LP);
            fn curry(self) -> Self::Output {
                impl_box_fn!(self, (LP), (lp), (lp))
            }
        }
    };
    (($LP:ident, $lp:ident), $(($P:ident, $p:ident)),*) => {
        impl<'a, F, O, $($P),*, LP> Curry<'a, (O, $($P),*, LP)> for F
        where
            F: Fn($($P),*, LP) -> O + 'a,
            $($P: Copy + 'a),*
        {
            type Output = impl_box_fn_ident!($($P),*, LP);
            fn curry(self) -> Self::Output {
                impl_box_fn!(self, ($($P),*, LP), ($($p),*, lp), ($($p),*, lp))
            }
        }
    };
}

all_tuples!(impl_curry, 1, 8, P, p_);
impl_curry!();

#[macro_export]
macro_rules! curry {
    ($e: expr) => {
        $e.curry()
    };
}

#[cfg(test)]
mod tests {
    use super::Curry;

    #[test]
    fn test_curry_macro() {
        let add = |&a, &b| a + b;
        let result = curry!(add)(&2);
        assert_eq!(result(&3), 5);
        assert_eq!(result(&3), 5);
    }
}

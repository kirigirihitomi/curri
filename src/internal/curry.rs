use bevy_utils::all_tuples;

pub trait Curry<'a, O> {
    type Output;
    fn curry(self) -> Self::Output;
}

macro_rules! impl_box_fn_ident {
    () => {Box<dyn Fn() -> O +'a>};
    ($P0:ident) => {Box<dyn Fn($P0) -> O +'a>};
    ($P0:ident, $($P:ident),*) =>{Box<dyn Fn($P0) -> impl_box_fn_ident!($($P),*)+'a>};
}

macro_rules! impl_box_fn {
    ($self:ident, (), (), ()) => {Box::new($self)};
    ($self:ident, ($P0:ident), ($p0:ident), ($($p:ident),*)) => {Box::new(move |$p0| $self($($p),*))};
    ($self:ident, ($P0:ident, $($P:ident),*) ,($p0:ident, $($p:ident),*), ($($p_:ident),*)) => {Box::new(move |$p0| impl_box_fn!($self, ($($P),*), ($($p),*), ($($p_),*)))};
}

macro_rules! impl_curry {
    ($(($P:ident, $p:ident)),*) => {
        impl<'a, F, O, $($P),*> Curry<'a, (O, $($P),*)> for F
        where
            F: Fn($($P),*) -> O + 'a + Copy,
            $($P: 'a + Copy),*
        {
            type Output = impl_box_fn_ident!($($P),*);
            fn curry(self) -> Self::Output {
                impl_box_fn!(self, ($($P),*), ($($p),*), ($($p),*))
            }
        }
    };
}

all_tuples!(impl_curry, 0, 8, P, p_);

#[macro_export]
macro_rules! curry {
    ($e: expr) => {
        $e.curry()
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_curry() {
        // Test case 1
        let add = |a, b| a + b;
        let curried_add = add.curry();
        let result = curried_add(2)(3);
        assert_eq!(result, 5);
        let result = curried_add(2)(3);
        assert_eq!(result, 5);

        // Test case 2
        let multiply = |a, b, c| a * b * c;
        let curried_multiply = multiply.curry();
        let result = curried_multiply(2)(3)(4);
        assert_eq!(result, 24);
    }
    #[test]
    fn test_curry_string() {
        // Test case 3
        let concat = |a: &str, b: &str| format!("{}{}", a, b);
        let curried_concat = concat.curry();
        let result = curried_concat("Hello")("World");
        assert_eq!(result, "HelloWorld");
    }

    #[test]
    fn test_curry_macro() {
        // Test case 4
        let add = |a, b| a + b;
        let result = curry!(add)(2)(3);
        assert_eq!(result, 5);
    }
}

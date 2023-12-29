#[macro_export]
macro_rules! impl_compose {
    (($f: expr), $t:expr) => {$f($t)};
    (($f: expr, $($fs: expr),*), $t:ident) => {
       $f(impl_compose!(($($fs),*), $t))
    };
}

#[macro_export]
macro_rules! compose {
    ($($f: expr),+) => {
        |t| impl_compose!(($($f),*),t)
    };
}

pub fn compose_vec<'a, T: 'a>(fs: Vec<Box<dyn Fn(T) -> T + 'a>>) -> Box<dyn Fn(T) -> T + 'a> {
    let mut fs = fs;
    let f = fs.pop().unwrap();
    let mut composed = f;
    for f in fs.into_iter().rev() {
        composed = Box::new(move |t| f(composed(t)));
    }
    composed
}

#[cfg(test)]
mod tests {
    use crate::Curry;

    use super::*;

    #[test]
    fn test_compose_macro() {
        let add_one = |x| x + 1;
        let double = |x| x * 2;
        let square = |x| x * x;

        let composed = compose!(add_one, double, square);
        assert_eq!(composed(2), 9);
    }

    #[test]
    fn test_compose_macro_curry() {
        let add_one = (|x: i32| x + 1).curry();
        let double = (|x: i32| x * 2).curry();
        let square = |x: i32| x * x;

        let composed = compose!(add_one, double, square);
        assert_eq!(composed(2), 9);
    }

    #[test]
    fn test_compose_vec() {
        let add_one = (|x: i32| x + 1).curry();
        let double = (|x: i32| x * 2).curry();
        let square = (|x: i32| x * x).curry();

        let composed = compose_vec(vec![add_one, double, square]);
        assert_eq!(composed(2), 9);
    }
}

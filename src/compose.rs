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
    #[test]
    fn test_compose_macro() {
        let add_one = |x| x + 1;
        let double = |x| x * 2;
        let square = |x| x * x;

        let composed = compose!(add_one, double, square);
        assert_eq!(composed(2), 9);
    }

    #[test]
    fn test_compose_macro_with_one_function() {
        let add_one = |x| x + 1;
        let composed = compose!(add_one);
        assert_eq!(composed(2), 3);
    }

    #[test]
    fn test_compose_macro_i32_to_string_concat_hello_world() {
        let to_string = |x: i32| x.to_string();
        let concat = |a: String| format!("{}{}", a, "HelloWorld");
        let composed = compose!(concat, to_string);
        assert_eq!(composed(2), "2HelloWorld");
    }
}

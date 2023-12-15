#[macro_export]
macro_rules! impl_compose {
    (($f: ident), $t:ident) => {$f($t)};
    (($f: ident, $($fs: ident),*), $t:ident) => {
       $f(impl_compose!(($($fs),*), $t))
    };
}

#[macro_export]
macro_rules! compose {
    ($($f: ident),+) => {
        |t| impl_compose!(($($f),*),t)
    };
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

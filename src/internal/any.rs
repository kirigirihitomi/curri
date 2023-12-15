#[macro_export]
macro_rules! any {
    ($check_list: ident) => {
        (Box::new(|t| {
            for f in &$check_list {
                if f(t) {
                    return true;
                }
            }
            false
        }))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_any_macro() {
        let check_list = vec![|x| x > 0, |x| x % 2 == 0];
        let any_fn = any!(check_list);

        assert_eq!(any_fn(5), true); // At least one function returns true
        assert_eq!(any_fn(-3), false); // No function returns true
        assert_eq!(any_fn(-2), true); // At least one function returns true
    }
}

#[macro_export]
macro_rules! all {
    ( $check_list: ident) => {
        (Box::new(|t| {
            for f in &$check_list {
                if !f(t) {
                    return false;
                }
            }
            true
        }))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_all_macro() {
        let check_list = vec![|x: i32| x > 0, |x: i32| x < 10];
        let all = all!(check_list);
        assert_eq!(all(5), true);
        assert_eq!(all(15), false);
    }
}

use crate::reduce;

pub fn any<'a, T, F>(check_list: Vec<F>) -> Box<dyn Fn(T) -> bool + 'a>
where
    T: Copy,
    F: Fn(T) -> bool + 'a,
{
    Box::new(move |t: T| reduce(|acc: bool, check: &F| acc || check(t))(false, &check_list))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curry::Curry;

    #[test]
    fn test_any_macro() {
        let check_list = vec![(|&x| x > 0).curry(), (|&x| x % 2 == 0).curry()];
        let any_fn = any(check_list);

        assert_eq!(any_fn(&5), true); // At least one function returns true
        assert_eq!(any_fn(&-3), false); // No function returns true
        assert_eq!(any_fn(&-2), true); // At least one function returns true
    }

    #[test]
    fn test_any_macro_1() {
        let check_list = vec![(|x| x > 0).curry(), (|x| x % 2 == 0).curry()];
        let any_fn = any(check_list);

        assert_eq!(any_fn(5), true); // At least one function returns true
        assert_eq!(any_fn(-3), false); // No function returns true
        assert_eq!(any_fn(-2), true); // At least one function returns true
    }
}

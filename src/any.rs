use crate::reduce;

#[doc = r"[a -> bool] -> (a -> bool)
Returns true if any of the predicates are satisfied by the given value, false otherwise.
# Example
```
use curri::any;
let fn1 = |&x| x > 0;
let fn2 = |&x| x % 2 == 0;
let check_list = vec![fn1, fn2];
let any_fn = any(check_list);
assert_eq!(any_fn(&5), true);
```
"]
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
    fn test_any() {
        let check_list = vec![(|&x| x > 0).curry(), (|&x| x % 2 == 0).curry()];
        let any_fn = any(check_list);

        assert_eq!(any_fn(&5), true); // At least one function returns true
        assert_eq!(any_fn(&-3), false); // No function returns true
        assert_eq!(any_fn(&-2), true); // At least one function returns true
    }
}

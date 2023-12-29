use crate::reduce;

#[doc = r"[a -> bool] -> (a -> bool)
Returns true if all the predicates are satisfied by the given value, false otherwise.
# Example
```
use curri::all;
let fn1 = |&x| x > 0;
let fn2 = |&x| x % 2 == 0;
let check_list = vec![fn1, fn2];
let all_fn = all(check_list);
assert_eq!(all_fn(&4), true);
```
"]
pub fn all<'a, T, F>(check_list: Vec<F>) -> Box<dyn Fn(T) -> bool + 'a>
where
    T: Copy,
    F: Fn(T) -> bool + 'a,
{
    Box::new(move |t: T| reduce(|acc: bool, check: &F| acc && check(t))(true, &check_list))
}

#[cfg(test)]
mod tests {
    use crate::Curry;

    use super::*;
    #[test]
    fn test_all() {
        let fn1 = |&x| x > 0;
        let fn2 = |&x| x % 2 == 0;
        let check_list = vec![fn1.curry(), fn2.curry()];
        let all_fn = all(check_list);
        assert_eq!(all_fn(&4), true);
        assert_eq!(all_fn(&4), true);
    }
}

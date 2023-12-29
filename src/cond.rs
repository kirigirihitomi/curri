use crate::reduce;

#[doc = r"[(a -> bool, a -> b)] -> (a -> Option<b>)
Returns the first value that satisfies the predicate, None otherwise.
# Example
```
use curri::{cond, Curry};
let iter = vec![
    ((|x: &i32| *x > 0).curry(), (|x: &i32| (*x * 2)).curry()),
    ((|x: &i32| *x < 0).curry(), (|x: &i32| (*x * 3)).curry()),
];
let cond_fn = cond(iter);
assert_eq!(cond_fn(&5), Some(10));
assert_eq!(cond_fn(&-5), Some(-15));
assert_eq!(cond_fn(&0), None);
```
"]
pub fn cond<'a, 'b: 'a, 'c: 'a, T, R, F, G>(iter: Vec<(F, G)>) -> Box<dyn Fn(T) -> Option<R> + 'a>
where
    T: Copy,
    F: Fn(T) -> bool + 'b,
    G: Fn(T) -> R + 'c,
{
    Box::new(move |value: T| {
        reduce(|acc: Option<R>, (predicate, transform): &(F, G)| match acc {
            Some(_) => acc,
            None => match predicate(value) {
                true => Some(transform(value)),
                false => None,
            },
        })(None, &iter)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curry::Curry;

    #[test]
    fn test_cond() {
        let iter = vec![
            ((|x: &i32| *x > 0).curry(), (|x: &i32| (*x * 2)).curry()),
            ((|x: &i32| *x < 0).curry(), (|x: &i32| (*x * 3)).curry()),
        ];
        let cond_fn = cond(iter);

        assert_eq!(cond_fn(&5), Some(10));
        assert_eq!(cond_fn(&-5), Some(-15));
        assert_eq!(cond_fn(&0), None);
    }
}

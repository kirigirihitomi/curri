use crate::reduce;

#[doc = r"((a -> bool), (a -> b)) -> ([a] -> Option b)
Returns the first value that satisfies the predicate, None otherwise.
# Example
```
use curri::{find, Curry};
let vec = vec![1, 2, 3, 4, 5];
let find_fn = |x: &i32| *x == 3;
let action_fn = |x: &i32| *x * 2;
let find_action = find(find_fn, action_fn);
assert_eq!(find_action(&vec), Some(6));
```
"]
pub fn find<'a, 'b: 'a, 'c: 'a, F: 'b, G: 'c, T, R>(find_fn: F, action_fn: G) -> Box<dyn Fn(&Vec<T>) -> Option<R> + 'a>
where
    F: Fn(&T) -> bool + 'b,
    G: Fn(&T) -> R + 'c,
{
    Box::new(move |v: &Vec<T>| {
        reduce(|acc: Option<R>, t: &T| match acc {
            Some(_) => acc,
            None => match find_fn(t) {
                true => Some(action_fn(t)),
                false => None,
            },
        })(None, v)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let vec = vec![1, 2, 3, 4, 5];
        let find_fn = |x: &i32| *x == 3;
        let action_fn = |x: &i32| *x * 2;
        let find_action = find(find_fn, action_fn);
        assert_eq!(find_action(&vec), Some(6));
    }

    #[test]
    fn test_find_none() {
        let vec = vec![1, 2, 3, 4, 5];
        let find_fn = |x: &i32| *x == 6;
        let action_fn = |x: &i32| *x * 2;
        let find_action = find(find_fn, action_fn);
        assert_eq!(find_action(&vec), None);
    }
}

fn reduce_fn<R, F, I>(f: F, r: R, mut v: I) -> R
where
    F: Fn(R, <I as Iterator>::Item) -> R + Copy,
    I: Iterator,
{
    match v.next() {
        Some(t) => reduce_fn(f, f(r, t), v),
        None => r,
    }
}

#[doc = r"[(a, b) -> a] -> (a, [b]) -> a
Returns the result of applying the function to each element of the list.
# Example
```
use curri::reduce;
let add = |a: i32, &b: &i32| a + b;
let reduce_add = reduce(add);
let arr = vec![1, 2, 3, 4, 5];
assert_eq!(reduce_add(0, &arr), 15);
```
"]
pub fn reduce<'a, R, F, T>(f: F) -> Box<dyn Fn(R, &Vec<T>) -> R + 'a>
where
    F: Fn(R, &T) -> R + Copy + 'a,
{
    Box::new(move |r, v| {
        let iter = v.iter();
        reduce_fn(f, r, iter)
    })
}

#[cfg(test)]
mod tests {
    use crate::Curry;

    use super::*;

    #[test]
    fn test_reduce_add() {
        let add = |a: i32, &b: &i32| a + b;
        let reduce_add = reduce(add);
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(reduce_add(0, &arr), 15);
    }

    #[test]
    fn test_reduce_on_return() {
        let arr = vec![1, 2, 3, 4, 5];
        fn inside() -> Box<dyn Fn(i32, &Vec<i32>) -> i32> {
            let add = |a: i32, &b: &i32| a + b;
            reduce(add)
        }
        assert_eq!(inside()(0, &arr), 15);
    }

    #[test]
    fn test_reduce_curry() {
        let add = |a: i32, &b: &i32| a + b;
        let reduce_add = reduce(add).curry();
        let arr = vec![1, 2, 3, 4, 5];
        let add_from_0 = reduce_add(0);
        assert_eq!(add_from_0(&arr), 15);
    }
}

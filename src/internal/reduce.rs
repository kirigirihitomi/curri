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

pub fn reduce<'a, 'b: 'a, R, F, T>(f: F) -> Box<dyn Fn(R, &Vec<T>) -> R + 'a>
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
    use super::*;

    #[test]
    fn test_reduce_addition() {
        let add = |a: i32, &b: &i32| a + b;
        let reduce_add = reduce(add);
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(reduce_add(0, &arr), 15);
    }

    #[test]
    fn test_reduce_multiplication() {
        let multiply = |a: i32, &b: &i32| a * b;
        let reduce_multiply = reduce(multiply);
        assert_eq!(reduce_multiply(1, &vec![1, 2, 3, 4, 5]), 120);
    }

    #[test]
    fn test_reduce_floating_point_addition() {
        let add = |a: f64, &b: &f64| a + b;
        let reduce_add = reduce(add);
        assert_eq!(reduce_add(0.0, &vec![1.0, 2.0, 3.0, 4.0, 5.0]), 15.0);
    }

    #[test]
    fn test_reduce_on_return() {
        fn inside() -> i32 {
            let add = |a: i32, &b: &i32| a + b;
            let reduce_add = reduce(add);
            let arr = vec![1, 2, 3, 4, 5];
            reduce_add(0, &arr)
        }
        assert_eq!(inside(), 15);
    }
}

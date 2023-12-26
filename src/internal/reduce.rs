pub fn reduce<'a, T, R, F>(f: F) -> Box<dyn Fn(R) -> Box<dyn Fn(Vec<T>) -> R + 'a> + 'a>
where
    T: Copy + 'a,
    R: Copy + 'a,
    F: Fn(R, T) -> R + Copy + 'a,
{
    Box::new(move |r| {
        Box::new(move |v| {
            let mut r = r;
            for t in v.iter() {
                r = f(r, *t);
            }
            r
        })
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_addition() {
        let add = |a: i32, b: i32| a + b;
        let reduce_add = reduce(add);
        assert_eq!(reduce_add(0)(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_reduce_multiplication() {
        let multiply = |a: i32, b: i32| a * b;
        let reduce_multiply = reduce(multiply);
        assert_eq!(reduce_multiply(1)(vec![1, 2, 3, 4, 5]), 120);
    }

    #[test]
    fn test_reduce_floating_point_addition() {
        let add = |a: f64, b: f64| a + b;
        let reduce_add = reduce(add);
        assert_eq!(reduce_add(0.0)(vec![1.0, 2.0, 3.0, 4.0, 5.0]), 15.0);
    }
}
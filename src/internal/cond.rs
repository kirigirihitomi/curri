pub fn cond<'a, T, R>(iter: &'a Vec<(Box<dyn Fn(T) -> bool>, Box<dyn Fn(T) -> R>)>) -> Box<dyn Fn(T) -> Option<R> + 'a>
where
    T: Copy,
{
    Box::new(move |value: T| {
        for (predicate, transform) in iter {
            if predicate(value) {
                return Some(transform(value));
            }
        }
        None
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::curry::Curry;

    #[test]
    fn test_cond() {
        let iter = vec![
            ((|x| x > 0).curry(), (|x| (x * 2)).curry()),
            ((|x| x < 0).curry(), (|x| (x * 3)).curry()),
        ];

        let cond_fn = cond(&iter);

        assert_eq!(cond_fn(5), Some(10));
        assert_eq!(cond_fn(-5), Some(-15));
        assert_eq!(cond_fn(0), None);
    }
}
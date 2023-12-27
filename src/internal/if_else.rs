pub fn if_else<'a, 'b: 'a, 'c: 'a, 'd: 'a, T, R, F, G, H>(check: F, if_fn: G, else_fn: H) -> Box<dyn Fn(T) -> R + 'a>
where
    F: Fn(&T) -> bool + 'b,
    G: Fn(T) -> R + 'c,
    H: Fn(T) -> R + 'd,
{
    Box::new(move |t: T| if check(&t) { if_fn(t) } else { else_fn(t) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Curry;

    #[test]
    fn test_if_else() {
        let check = |&x: &i32| x > 0;
        let if_fn = |x: i32| x * 2;
        let else_fn = |x: i32| x * 3;

        let result = if_else(check, if_fn.curry(), else_fn);
        assert_eq!(result(5), 10);
        assert_eq!(result(-5), -15);
        assert_eq!(result(0), 0);
    }
}

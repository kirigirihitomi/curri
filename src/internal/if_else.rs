pub fn if_else<'a, T, R, F, G, H>(check: F, if_fn: G, else_fn: H) -> Box<dyn Fn(T) -> R + 'a>
where
    T: 'a + Copy,
    F: Fn(T) -> bool + 'a,
    G: Fn(T) -> R + 'a,
    H: Fn(T) -> R + 'a,
{
    Box::new(move |t: T| if check(t) { if_fn(t) } else { else_fn(t) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_else() {
        let check = |&x: &i32| x > 0;
        let if_fn = |&x: &i32| x * 2;
        let else_fn = |&x: &i32| x * 3;

        let result = if_else(check, if_fn, else_fn);

        assert_eq!(result(&5), 10);
        assert_eq!(result(&-5), -15);
        assert_eq!(result(&0), 0);
    }
}

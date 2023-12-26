pub fn if_else<'a, T, R>(check: Box<dyn Fn(&T) -> bool>, if_fn: Box<dyn Fn(&T) -> R>, else_fn: Box<dyn Fn(&T) -> R>) -> Box<dyn Fn(&T) -> R + 'a>
where
    T: 'a,
    R: 'a,
{
    Box::new(move |t: &T| if check(t) { if_fn(t) } else { else_fn(t) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_else() {
        let check = Box::new(|x: &i32| x > &0);
        let if_fn = Box::new(|x: &i32| x * 2);
        let else_fn = Box::new(|x: &i32| x * 3);

        let result = if_else(check, if_fn, else_fn);

        assert_eq!(result(&5), 10);
        assert_eq!(result(&-5), -15);
        assert_eq!(result(&0), 0);
    }
}

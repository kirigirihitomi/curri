pub fn always<'a, T>(t: T) -> Box<dyn Fn() -> T + 'a>
where
    T: Copy + 'a,
{
    Box::new(move || t.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always() {
        let value = 42;
        let always_fn = always(value);

        assert_eq!(always_fn(), value);
        assert_eq!(always_fn(), value);
        assert_eq!(always_fn(), value);
        assert_eq!(always_fn(), value);
    }
}

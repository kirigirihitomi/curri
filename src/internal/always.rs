pub fn always<T>(t: T) -> Box<dyn Fn() -> T>
where
    T: Clone + 'static,
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

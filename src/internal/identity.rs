pub fn identity<T>(x: T) -> T {
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        assert_eq!(identity(5), 5);
        assert_eq!(identity("hello"), "hello");
        assert_eq!(identity(vec![1, 2, 3]), vec![1, 2, 3]);
    }
}

#[doc = r"
a -> (* -> a)
Returns a function that always returns the given value.
# Example
```
use curri::always;
let always_42 = always(42);
assert_eq!(always_42(), 42);
```
"]
pub fn always<'a, T: 'a>(t: T) -> Box<dyn Fn() -> T + 'a>
where
    T: Clone,
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
    }
}

pub fn map<'a, T, R>(f: Box<dyn Fn(&'a T) -> R + 'a>) -> Box<dyn Fn(&'a Vec<T>) -> Vec<R> + 'a>
where
    R: 'a,
{
    Box::new(move |v| {
        let mut r = vec![];
        for t in v {
            r.push(f(t));
        }
        r
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let input = vec![1, 2, 3];
        let expected = vec![2, 4, 6];
        let double = Box::new(|x: &i32| x * 2);
        let map_double = map(double);
        assert_eq!(map_double(&input), expected);
    }
}

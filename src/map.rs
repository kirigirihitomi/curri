use crate::reduce;

pub fn map<'a, T, R, F>(f: F) -> Box<dyn Fn(&Vec<T>) -> Vec<R> + 'a>
where
    T: Copy,
    F: Fn(&T) -> R + 'a,
{
    Box::new(move |v| {
        let r = vec![];
        reduce(|mut acc: Vec<R>, t: &T| {
            acc.push(f(t));
            acc
        })(r, v)
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let input = vec![1, 2, 3];
        let expected = vec![2, 4, 6];
        let double = |&x: &i32| x * 2;
        let map_double = map(double);
        assert_eq!(map_double(&input), expected);
    }
}

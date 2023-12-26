pub fn all<'a, T>(check_list: Vec<Box<dyn Fn(T) -> bool>>) -> Box<dyn Fn(T) -> bool + 'a>
where
    T: 'a + Copy,
{
    Box::new(move |t: T| {
        let mut result = true;
        for check in check_list.iter() {
            result = result && check(t);
        }
        result
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let check_list: Vec<Box<dyn Fn(&i32) -> bool>> = vec![Box::new(|&x| x > 0), Box::new(|&x| x % 2 == 0)];

        let all_fn = all(check_list);

        assert_eq!(all_fn(&4), true);
        assert_eq!(all_fn(&-2), false);
        assert_eq!(all_fn(&7), false);
    }
}

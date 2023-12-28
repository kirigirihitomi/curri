use crate::reduce;

pub fn all<'a, T, F>(check_list: Vec<F>) -> Box<dyn Fn(T) -> bool + 'a>
where
    T: Copy,
    F: Fn(T) -> bool + 'a,
{
    Box::new(move |t: T| reduce(|acc: bool, check: &F| acc && check(t))(true, &check_list))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestStruct {
        pub value: i32,
    }

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            println!("Clone");
            TestStruct { value: self.value.clone() }
        }
    }

    impl Copy for TestStruct {}

    #[test]
    fn test_all() {
        let fn1 = |&x| x > 0;
        let fn2 = |&x| x % 2 == 0;
        let check_list = vec![fn1, fn2];
        let all_fn = all(check_list);
        assert_eq!(all_fn(&4), true);
        assert_eq!(all_fn(&4), true);
    }

    #[test]
    fn test_for_test_struct<'a>() {
        {
            let test_struct = TestStruct { value: 4 };
            let check_list = vec![(|x: &TestStruct| x.value > 0), (|x: &TestStruct| x.value % 2 == 0)];
            let all_fn = all(check_list);
            let result = all_fn(&test_struct);
            assert_eq!(result, true);
        }
    }
}

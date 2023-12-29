pub fn t<T>(_: T) -> bool {
    true
}

pub fn t_ref<T>(_: &T) -> bool {
    true
}

pub fn f<T>(_: T) -> bool {
    false
}

pub fn f_ref<T>(_: &T) -> bool {
    false
}

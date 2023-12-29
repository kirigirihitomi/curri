#[doc = r"(*) -> true
Returns true.
# Example
```
use curri::t;
assert_eq!(t(1), true);
```
"]
pub fn t<T>(_: T) -> bool {
    true
}

#[doc = r"(&*) -> true
Returns true.
# Example
```
use curri::t_ref;
assert_eq!(t_ref(&1), true);
```
"]
pub fn t_ref<T>(_: &T) -> bool {
    true
}

#[doc = r"(*) -> false
Returns false.
# Example
```
use curri::f;
assert_eq!(f(1), false);
```
"]
pub fn f<T>(_: T) -> bool {
    false
}

#[doc = r"(&*) -> false
Returns false.
# Example
```
use curri::f_ref;
assert_eq!(f_ref(&1), false);
```
"]
pub fn f_ref<T>(_: &T) -> bool {
    false
}

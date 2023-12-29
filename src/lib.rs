 mod all;
 mod always;
 mod any;
 mod apply;
 mod assoc;
 mod bool;
 mod compose;
 mod cond;
 mod curry;
 mod find;
 mod identity;
 mod if_else;
 mod map;
 mod reduce;    

pub use all::*;
pub use always::*;
pub use any::*;
pub use apply::*;
pub use assoc::*;
pub use bool::*;
pub use compose::*;
pub use cond::*;
pub use curry::*;
pub use find::*;
pub use identity::*;
pub use if_else::*;
pub use map::*;
pub use reduce::*;
#[cfg(test)]
mod tests {
    #[test]
    fn test_internal() {}
}

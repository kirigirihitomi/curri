#[macro_export]
macro_rules! assoc {
    ($T: ident, $($p: ident),*) => {
        (move |p| Box::new( move |mut t: $T|{ t$(.$p)* = p; t}))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_assoc_macro() {
        struct X {
            y: i32,
        }
        let x = X { y: 1 };
        let y = assoc!(X, y)(10);
        assert_eq!(y(x).y, 10);
        let x = X { y: 1 };
        assert_eq!(y(x).y, 10);
    }

    #[test]
    fn test_assoc_macro_path() {
        struct Y {
            z: i32,
        }
        struct X {
            y: Y,
        }
        let x = X { y: Y { z: 1 } };
        let y = assoc!(X, y, z)(10);
        assert_eq!(y(x).y.z, 10);
    }
}

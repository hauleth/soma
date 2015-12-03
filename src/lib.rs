#[macro_export]
/// Helper macro for unwrapping `Option` values while returning early from function if value
/// is `None`. `try_some!(expr)` should be used only if function return type is `Option` and
/// `try_some!(expr => return)` should be used only if function return type is `()`.
macro_rules! try_some {
    ($expr:expr) => (match $expr {
        ::std::option::Option::Some(val) => val,
        ::std::option::Option::None => return ::std::option::Option::None,
    });

    ($expr:expr => return) => (match $expr {
        ::std::option::Option::Some(val) => val,
        ::std::option::Option::None => return,
    })
}

#[cfg(test)]
mod test {
    fn some(o: Option<()>) -> Option<()> {
        Some(try_some!(o))
    }

    #[test]
    fn value_on_some() {
        assert_eq!(some(Some(())), Some(()));
    }

    #[test]
    fn none_on_none() {
        assert_eq!(some(None), None);
    }

    #[test]
    fn abort_on_none() {
        fn some(o: Option<()>) {
            try_some!(o => return);

            assert!(false, "This should not happen!")
        }

        some(None)
    }

    #[test]
    #[should_panic]
    fn dont_abort_on_some() {
        fn some(o: Option<()>) {
            try_some!(o => return);

            assert!(false, "This should happen!")
        }

        some(Some(()))
    }
}

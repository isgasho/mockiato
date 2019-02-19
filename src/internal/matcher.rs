use crate::internal::arguments::Arguments;
use crate::internal::fmt::MaybeDebug;
use std::fmt::Debug;

pub(crate) mod any;
pub(crate) mod partial_eq;
pub(crate) mod unordered_vec_eq;

pub trait ArgumentMatcher<T>: MaybeDebug {
    fn matches_argument(&self, input: &T) -> bool;
}

pub trait ArgumentsMatcher<'args>: Debug {
    type Arguments: Arguments;

    fn matches_arguments(&self, _input: &Self::Arguments) -> bool {
        // Since argument matchers for methods without any arguments should always match, we can
        // fall back to the default impl on the trait `ArgumentsMatcher`.
        true
    }
}

#[cfg(test)]
pub(crate) use self::mock::*;

#[cfg(test)]
mod mock {
    use super::ArgumentsMatcher;
    use crate::internal::arguments::ArgumentsMock;
    use std::cell::RefCell;
    use std::thread::panicking;

    #[derive(Debug, Default)]
    pub(crate) struct ArgumentsMatcherMock {
        matches_arguments_return: Option<bool>,
        matches_arguments_was_called: RefCell<bool>,
    }

    impl ArgumentsMatcherMock {
        pub(crate) fn new(matches_arguments_return: Option<bool>) -> Self {
            Self {
                matches_arguments_return,
                matches_arguments_was_called: RefCell::new(false),
            }
        }
    }

    impl<'args> ArgumentsMatcher<'args> for ArgumentsMatcherMock {
        type Arguments = ArgumentsMock;

        fn matches_arguments(&self, _input: &Self::Arguments) -> bool {
            *self.matches_arguments_was_called.borrow_mut() = true;
            self.matches_arguments_return.unwrap()
        }
    }

    impl Drop for ArgumentsMatcherMock {
        fn drop(&mut self) {
            if !panicking() && self.matches_arguments_return.is_some() {
                assert!(
                    *self.matches_arguments_was_called.borrow(),
                    "matches_arguments() was never called"
                );
            }
        }
    }
}

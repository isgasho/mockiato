use crate::internal::debug::MaybeDebug;
use crate::internal::matcher::ArgumentsMatcher;
use std::fmt::{self, Debug, Display};

pub trait DefaultReturnValue {
    fn default_return_value<A>() -> Option<Box<dyn ReturnValueGenerator<A, Self>>>
    where
        Self: Sized,
        A: for<'args> ArgumentsMatcher<'args>;
}

impl<T> DefaultReturnValue for T {
    default fn default_return_value<A>() -> Option<Box<dyn ReturnValueGenerator<A, T>>>
    where
        Self: Sized,
        A: for<'args> ArgumentsMatcher<'args>,
    {
        None
    }
}

impl DefaultReturnValue for () {
    fn default_return_value<A>() -> Option<Box<dyn ReturnValueGenerator<A, ()>>>
    where
        Self: Sized,
        A: for<'args> ArgumentsMatcher<'args>,
    {
        Some(Box::new(Cloned(())))
    }
}

pub trait ReturnValueGenerator<A, R>: Display + Debug
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    fn generate_return_value(&self, input: <A as ArgumentsMatcher>::Arguments) -> R;
}

pub struct Cloned<T>(pub(crate) T)
where
    T: Clone + MaybeDebug;

impl<R> Display for Cloned<R>
where
    R: Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MaybeDebug::fmt(&self.0, f)
    }
}

impl<R> Debug for Cloned<R>
where
    R: Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MaybeDebug::fmt(&self.0, f)
    }
}

impl<A, R> ReturnValueGenerator<A, R> for Cloned<R>
where
    A: for<'args> ArgumentsMatcher<'args>,
    R: Clone,
{
    fn generate_return_value(&self, _: <A as ArgumentsMatcher>::Arguments) -> R {
        self.0.clone()
    }
}

#[derive(Debug)]
pub struct Panic(pub(crate) Option<&'static str>);

impl Display for Panic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "panic!(")?;

        if let Some(message) = self.0 {
            write!(f, "{:?}", message)?;
        }

        write!(f, ")")
    }
}

impl<A, R> ReturnValueGenerator<A, R> for Panic
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    fn generate_return_value(&self, _: <A as ArgumentsMatcher>::Arguments) -> R {
        match self.0 {
            Some(message) => panic!(message),
            None => panic!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::internal::arguments::Arguments;

    #[derive(Debug)]
    struct DummyArguments;

    impl Arguments for DummyArguments {}

    #[derive(Debug, Default)]
    struct DummyArgumentsMatcher;

    impl<'args> ArgumentsMatcher<'args> for DummyArgumentsMatcher {
        type Arguments = DummyArguments;

        fn matches_arguments(&self, _input: &Self::Arguments) -> bool {
            unimplemented!();
        }
    }

    #[test]
    #[should_panic(expected = "<panic message>")]
    fn test_panic_panicks() {
        let panic = Panic(Some("<panic message>"));

        ReturnValueGenerator::<DummyArgumentsMatcher, ()>::generate_return_value(
            &panic,
            DummyArguments,
        );
    }

    #[test]
    fn test_cloned_returns_expected_value() {
        let cloned = Cloned(String::from("foo"));

        assert_eq!(
            String::from("foo"),
            ReturnValueGenerator::<DummyArgumentsMatcher, String>::generate_return_value(
                &cloned,
                DummyArguments
            )
        );
    }
}

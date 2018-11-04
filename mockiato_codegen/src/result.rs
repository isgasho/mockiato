use proc_macro::Diagnostic;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub(crate) enum Error {
    Empty,
    Diagnostic(Diagnostic),
    MultipleDiagnostics(Vec<Diagnostic>),
}

impl Error {
    pub(crate) fn emit<F>(self, map_fn: F) -> Self
    where
        F: Fn(Diagnostic) -> Diagnostic,
    {
        match self {
            Error::Empty => {}
            Error::Diagnostic(diagnostic) => map_fn(diagnostic).emit(),
            Error::MultipleDiagnostics(diagnostics) => {
                diagnostics.into_iter().for_each(|d| map_fn(d).emit());
            }
        };

        Error::Empty
    }

    pub(crate) fn merge<I>(errors: I) -> Self
    where
        I: Iterator<Item = Error>,
    {
        let mut collected = Vec::new();

        errors.for_each(|err| match err {
            Error::Empty => {}
            Error::Diagnostic(diagnostic) => collected.push(diagnostic),
            Error::MultipleDiagnostics(mut diagnostics) => collected.append(&mut diagnostics),
        });

        Error::MultipleDiagnostics(collected)
    }
}

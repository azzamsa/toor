use miette::Diagnostic;
use thiserror::Error;

/// all possible errors returned by the app.
#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("{0}")]
    Internal(String),

    #[error("Project root is not found.")]
    #[diagnostic(
        code(toor::no_project_root),
        url(docsrs),
        help("Make sure the project root exists.")
    )]
    RootNotFound,
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

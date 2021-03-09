use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError<R>
where
    R: Debug,
{
    #[error("Invalid placeholder: {0:?}")]
    InvalidPlaceholder(R),
    #[error("Missing placeholder: {0}")]
    MissingPlaceholder(String),
    #[error("Unclosed placeholder")]
    UnclosedPlaceholder,
    #[error("Unclosed quoted literal")]
    UnclosedQuotedLiteral,
}

impl<R> From<R> for ParserError<R>
where
    R: Debug,
{
    fn from(input: R) -> Self {
        Self::InvalidPlaceholder(input)
    }
}

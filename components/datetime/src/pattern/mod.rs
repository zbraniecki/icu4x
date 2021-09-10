mod error;
mod item;
mod parser;

use error::Error;
pub use item::*;
use parser::Parser;
use zerovec::ZeroVec;

#[derive(Clone, Debug, PartialEq)]
pub struct ZVPattern<'data>(pub ZeroVec<'data, PatternItem>);

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Pattern {
    items: Vec<PatternItem>,
}

impl From<ZVPattern<'_>> for Pattern {
    fn from(input: ZVPattern<'_>) -> Self {
        Self {
            items: input.0.to_vec(),
        }
    }
}

impl Pattern {
    pub fn items(&self) -> &[PatternItem] {
        &self.items
    }

    pub fn from_bytes(input: &str) -> Result<Self, Error> {
        Parser::new(input).parse().map(Self::from)
    }

    // TODO(#277): This should be turned into a utility for all ICU4X.
    pub fn from_bytes_combination(input: &str, date: Self, time: Self) -> Result<Self, Error> {
        Parser::new(input)
            .parse_placeholders(vec![time, date])
            .map(Self::from)
    }
}

impl From<Vec<PatternItem>> for Pattern {
    fn from(items: Vec<PatternItem>) -> Self {
        Self { items }
    }
}

impl From<&Pattern> for ZVPattern<'_> {
    fn from(input: &Pattern) -> Self {
        Self(ZeroVec::clone_from_slice(&input.items))
    }
}

mod item;

pub use item::*;
use zerovec::ZeroVec;

#[derive(Clone, Debug, PartialEq)]
pub struct ZVPattern<'data>(pub ZeroVec<'data, PatternItem>);

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern(pub Vec<PatternItem>);

impl From<ZVPattern<'_>> for Pattern {
    fn from(input: ZVPattern<'_>) -> Self {
        Self(input.0.to_vec())
    }
}

impl From<&Pattern> for ZVPattern<'_> {
    fn from(input: &Pattern) -> Self {
        Self(ZeroVec::clone_from_slice(&input.0))
    }
}

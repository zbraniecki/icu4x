mod error;
mod item;
mod parser;

use error::Error;
pub use item::*;
use parser::Parser;
use serde::{Deserialize, Serialize};
use zerovec::{ule::AsULE, ZeroVec};

pub trait PatternItemsVecType {
    type Iter: Iterator<Item = PatternItem>;

    fn get_items(&self) -> Self::Iter;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pattern<I> {
    pub items: I,
}

pub type VecPattern = Pattern<Vec<PatternItem>>;
pub type ZVPattern<'a> = Pattern<ZeroVec<'a, PatternItem>>;

impl Pattern<Vec<PatternItem>> {
    pub fn from_bytes(input: &str) -> Result<Self, Error> {
        let items = Parser::new(input).parse()?;
        Ok(Self { items })
    }

    // TODO(#277): This should be turned into a utility for all ICU4X.
    pub fn from_bytes_combination(input: &str, date: Self, time: Self) -> Result<Self, Error> {
        let items = Parser::new(input).parse_placeholders(vec![time, date])?;
        Ok(Self { items })
    }
}

impl<'a> PatternItemsVecType for &'a Pattern<Vec<PatternItem>> {
    type Iter = std::iter::Copied<std::slice::Iter<'a, PatternItem>>;

    fn get_items(&self) -> Self::Iter {
        self.items.iter().copied()
    }
}

pub struct ZeroVecIter<'a, T>
where
    T: AsULE,
{
    pub vec: &'a ZeroVec<'a, T>,
    pub idx: usize,
}

impl<'a, T> Iterator for ZeroVecIter<'a, T>
where
    T: AsULE + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.vec.get(self.idx)?;
        self.idx += 1;
        Some(result)
    }
}

impl<'a> PatternItemsVecType for &'a Pattern<ZeroVec<'static, PatternItem>> {
    type Iter = ZeroVecIter<'a, PatternItem>;

    fn get_items(&self) -> Self::Iter {
        ZeroVecIter {
            vec: &self.items,
            idx: 0,
        }
    }
}

// #[derive(Clone, Debug, PartialEq)]
// pub struct ZVPattern(pub ZeroVec<'static, PatternItem>);

// #[derive(Clone, Debug, PartialEq)]
// #[cfg_attr(
//     feature = "provider_serde",
//     derive(serde::Serialize, serde::Deserialize)
// )]
// pub struct Pattern {
//     pub items: Vec<PatternItem>,
// }

// impl From<ZVPattern> for Pattern {
//     fn from(input: ZVPattern) -> Self {
//         Self {
//             items: input.0.to_vec(),
//         }
//     }
// }

// impl From<Vec<PatternItem>> for Pattern {
//     fn from(items: Vec<PatternItem>) -> Self {
//         Self { items }
//     }
// }

impl From<&VecPattern> for ZVPattern<'_> {
    fn from(input: &VecPattern) -> Self {
        Self {
            items: ZeroVec::clone_from_slice(&input.items),
        }
    }
}

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
mod error;
pub mod parser;

use crate::fields::{self, Field, FieldLength, FieldSymbol};
pub use error::Error;
use icu_simple_formatter::{interpolate, parse, Element};
use icu_string::Slice;
use parser::Parser;
use std::borrow::Cow;
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq)]
pub enum PatternItem<'p> {
    Field(fields::Field),
    Literal(Cow<'p, str>),
}

impl<'p> From<Cow<'p, str>> for PatternItem<'p> {
    fn from(input: Cow<'p, str>) -> Self {
        Self::Literal(input)
    }
}

/// The granularity of time represented in a pattern item.
/// Ordered from least granular to most granular for comparsion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum TimeGranularity {
    Hours,
    Minutes,
    Seconds,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Pattern<'p> {
    pub items: Vec<PatternItem<'p>>,
    time_granularity: Option<TimeGranularity>,
}

/// Retrieves the granularity of time represented by a `PatternItem`.
/// If the `PatternItem` is not time-related, returns `None`.
fn get_time_granularity(item: &PatternItem) -> Option<TimeGranularity> {
    match item {
        PatternItem::Field(field) => match field.symbol {
            fields::FieldSymbol::Hour(_) => Some(TimeGranularity::Hours),
            fields::FieldSymbol::Minute => Some(TimeGranularity::Minutes),
            fields::FieldSymbol::Second(_) => Some(TimeGranularity::Seconds),
            _ => None,
        },
        _ => None,
    }
}

impl<'p> Pattern<'p> {
    pub fn items(&self) -> &[PatternItem] {
        &self.items
    }

    pub fn from_bytes(input: &Cow<'p, str>) -> Result<Self, Error> {
        let mut items = vec![];
        let mut parser = Parser::new(input);
        while let Some(item) = parser.next()? {
            items.push(item);
        }

        Ok(Self {
            time_granularity: None,
            items,
        })
    }

    // TODO(#277): This should be turned into a utility for all ICU4X.
    pub fn from_bytes_combination(
        input: &Cow<'p, str>,
        date: Self,
        time: Self,
    ) -> Result<Self, Error> {
        let i = icu_pattern::Parser::new(input);
        let mut pi = icu_pattern::Interpolator::new(i, vec![time.items, date.items]);
        let mut result = vec![];
        while let Some(elem) = pi.try_next().unwrap() {
            result.push(elem);
        }
        return Ok(result.into());
    }

    pub(super) fn most_granular_time(&self) -> Option<TimeGranularity> {
        self.time_granularity
    }
}

impl<'p> From<Vec<PatternItem<'p>>> for Pattern<'p> {
    fn from(items: Vec<PatternItem<'p>>) -> Self {
        Self {
            time_granularity: items.iter().filter_map(get_time_granularity).max(),
            items,
        }
    }
}

impl<'p> FromIterator<PatternItem<'p>> for Pattern<'p> {
    fn from_iter<I: IntoIterator<Item = PatternItem<'p>>>(iter: I) -> Self {
        Self::from(iter.into_iter().collect::<Vec<_>>())
    }
}

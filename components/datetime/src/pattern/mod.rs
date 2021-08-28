// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod error;
mod parser;
pub mod transform_hour_cycle;

use crate::fields::{self, Field, FieldLength, FieldSymbol};
#[cfg(feature = "provider_serde")]
use alloc::format;
use alloc::string::String;
#[cfg(feature = "provider_serde")]
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use core::{convert::TryFrom, fmt};
use core::{fmt::Write, iter::FromIterator};
pub use error::Error;
use parser::Parser;

#[cfg(feature = "provider_serde")]
use serde::{
    de,
    ser::{self, SerializeSeq},
    Deserialize, Deserializer, Serialize,
};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum PatternItem<'data> {
    Field(fields::Field),
    Literal(std::borrow::Cow<'data, str>),
}

impl From<(FieldSymbol, FieldLength)> for PatternItem<'_> {
    fn from(input: (FieldSymbol, FieldLength)) -> Self {
        Self::Field(Field {
            symbol: input.0,
            length: input.1,
        })
    }
}

impl TryFrom<(FieldSymbol, u8)> for PatternItem<'_> {
    type Error = Error;
    fn try_from(input: (FieldSymbol, u8)) -> Result<Self, Self::Error> {
        let length =
            FieldLength::try_from(input.1).map_err(|_| Error::FieldLengthInvalid(input.0))?;
        Ok(Self::Field(Field {
            symbol: input.0,
            length,
        }))
    }
}

impl<'data> From<&'data str> for PatternItem<'data> {
    fn from(input: &'data str) -> Self {
        Self::Literal(input.into())
    }
}

impl From<String> for PatternItem<'_> {
    fn from(input: String) -> Self {
        Self::Literal(input.into())
    }
}

/// The granularity of time represented in a pattern item.
/// Ordered from least granular to most granular for comparsion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub(super) enum TimeGranularity {
    Hours,
    Minutes,
    Seconds,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Pattern<'data> {
    items: Vec<PatternItem<'data>>,
    time_granularity: Option<TimeGranularity>,
}

/// Retrieves the granularity of time represented by a [`PatternItem`].
/// If the [`PatternItem`] is not time-related, returns [`None`].
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

impl<'data> Pattern<'data> {
    pub fn items(&self) -> &[PatternItem<'data>] {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut [PatternItem<'data>] {
        &mut self.items
    }

    pub fn from_bytes(input: &'data str) -> Result<Self, Error> {
        Parser::new(input).parse().map(Self::from)
    }

    // TODO(#277): This should be turned into a utility for all ICU4X.
    pub fn from_bytes_combination(
        input: &'data str,
        date: Self,
        time: Self,
    ) -> Result<Self, Error> {
        Parser::new(input)
            .parse_placeholders(vec![time, date])
            .map(Self::from)
    }

    pub(super) fn most_granular_time(&self) -> Option<TimeGranularity> {
        self.time_granularity
    }
}

impl<'data> From<Vec<PatternItem<'data>>> for Pattern<'data> {
    fn from(items: Vec<PatternItem<'data>>) -> Self {
        Self {
            time_granularity: items.iter().filter_map(get_time_granularity).max(),
            items,
        }
    }
}

/// This trait is implemented in order to provide the machinery to convert a [`Pattern`] to a UTS 35
/// pattern string. It could also be implemented as the Writeable trait, but at the time of writing
/// this was not done, as this code would need to implement the [`write_len()`] method, which would
/// need to duplicate the branching logic of the [`fmt`](std::fmt) method here. This code is used in generating
/// the data providers and is not as performance sensitive.
impl<'data> fmt::Display for Pattern<'data> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for pattern_item in self.items().iter() {
            match pattern_item {
                PatternItem::Field(field) => {
                    let ch: char = field.symbol.into();
                    for _ in 0..field.length as usize {
                        formatter.write_char(ch)?;
                    }
                }
                PatternItem::Literal(literal) => {
                    // Determine if the literal contains any characters that would need to be escaped.
                    let mut needs_escaping = false;
                    for ch in literal.chars() {
                        if ch.is_ascii_alphabetic() || ch == '\'' {
                            needs_escaping = true;
                            break;
                        }
                    }

                    if needs_escaping {
                        let mut ch_iter = literal.trim_end().chars().peekable();

                        // Do not escape the leading whitespace.
                        while let Some(ch) = ch_iter.peek() {
                            if ch.is_whitespace() {
                                formatter.write_char(*ch)?;
                                ch_iter.next();
                            } else {
                                break;
                            }
                        }

                        // Wrap in "'" and escape "'".
                        formatter.write_char('\'')?;
                        for ch in ch_iter {
                            if ch == '\'' {
                                // Escape a single quote.
                                formatter.write_char('\\')?;
                            }
                            formatter.write_char(ch)?;
                        }
                        formatter.write_char('\'')?;

                        // Add the trailing whitespace
                        for ch in literal.chars().rev() {
                            if ch.is_whitespace() {
                                formatter.write_char(ch)?;
                            } else {
                                break;
                            }
                        }
                    } else {
                        formatter.write_str(literal)?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

impl<'data> FromIterator<PatternItem<'data>> for Pattern<'data> {
    fn from_iter<I: IntoIterator<Item = PatternItem<'data>>>(iter: I) -> Self {
        Self::from(iter.into_iter().collect::<Vec<_>>())
    }
}

#[cfg(feature = "provider_serde")]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default)]
struct DeserializePatternUTS35String<'data>(std::marker::PhantomData<&'data str>);

#[cfg(feature = "provider_serde")]
impl<'de: 'data, 'data> de::Visitor<'de> for DeserializePatternUTS35String<'data> {
    type Value = Pattern<'data>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Expected to find a valid pattern.")
    }

    fn visit_borrowed_str<E>(self, pattern_string: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        // Parse a string into a list of fields.
        Pattern::from_bytes(pattern_string).map_err(|err| {
            de::Error::invalid_value(
                de::Unexpected::Other(&format!("{}", err)),
                &"a valid UTS 35 pattern string",
            )
        })
    }
}

#[cfg(feature = "provider_serde")]
#[derive(Default)]
struct DeserializePatternBincode<'data>(std::marker::PhantomData<&'data str>);

#[cfg(feature = "provider_serde")]
impl<'de, 'data> de::Visitor<'de> for DeserializePatternBincode<'data> {
    type Value = Pattern<'data>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Unable to deserialize a bincode Pattern.")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Pattern<'data>, V::Error>
    where
        V: de::SeqAccess<'de>,
    {
        let mut items = Vec::new();
        while let Some(item) = seq.next_element()? {
            items.push(item)
        }
        Ok(Pattern::from(items))
    }
}

#[cfg(feature = "provider_serde")]
impl<'de: 'data, 'data> Deserialize<'de> for Pattern<'data> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(DeserializePatternUTS35String::default())
        } else {
            deserializer.deserialize_seq(DeserializePatternBincode::default())
        }
    }
}

#[cfg(feature = "provider_serde")]
impl<'data> Serialize for Pattern<'data> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if serializer.is_human_readable() {
            // Serialize into the UTS 35 string representation.
            let string: String = self.to_string();
            serializer.serialize_str(&string)
        } else {
            // Serialize into a bincode-friendly representation. This means that pattern parsing
            // will not be needed when deserializing.
            let mut seq = serializer.serialize_seq(Some(self.items.len()))?;
            for item in self.items.iter() {
                seq.serialize_element(item)?;
            }
            seq.end()
        }
    }
}

/// Used to represent either H11/H12, or H23/H24. Skeletons only store these
/// hour cycles as H12 or H23.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum CoarseHourCycle {
    /// Can either be fields::Hour::H11 or fields::Hour::H12
    H11H12,
    /// Can either be fields::Hour::H23 or fields::Hour::H24
    H23H24,
}

/// Default is required for serialization. H23H24 is the more locale-agnostic choice, as it's
/// less likely to have a day period in it.
impl Default for CoarseHourCycle {
    fn default() -> Self {
        CoarseHourCycle::H23H24
    }
}

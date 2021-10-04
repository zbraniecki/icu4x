// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    super::{reference, PatternError},
    super::{GenericPatternItem, PatternItem},
    Pattern,
};
use alloc::vec::Vec;
use icu_provider::yoke::{self, Yokeable, ZeroCopyFrom};
use zerovec::{ule::AsULE, ZeroVec};

#[derive(Debug, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct GenericPattern<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub items: ZeroVec<'data, GenericPatternItem>,
}

impl Default for GenericPattern<'_> {
    fn default() -> Self {
        Self {
            items: ZeroVec::clone_from_slice(&[]),
        }
    }
}

impl<'data> GenericPattern<'data> {
    /// The function allows for creation of new DTF pattern from a generic pattern
    /// and replacement patterns.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_datetime::pattern::{runtime, reference};
    ///
    /// let date: runtime::Pattern =
    ///     reference::Pattern::from_bytes("Y-m-d")
    ///         .expect("Failed to parse pattern")
    ///         .into();
    /// let time: runtime::Pattern =
    ///     reference::Pattern::from_bytes("HH:mm")
    ///         .expect("Failed to parse pattern")
    ///         .into();
    ///
    /// let glue: runtime::GenericPattern =
    ///     reference::GenericPattern::from_bytes("{0} 'at' {1}")
    ///         .expect("Failed to parse generic pattern")
    ///         .into();
    /// assert_eq!(
    ///     glue.combined(vec![date, time])
    ///         .expect("Failed to combine patterns")
    ///         .to_string(),
    ///     "Y-m-d 'at' HH:mm"
    /// );
    /// ```
    pub fn combined(
        self,
        date: &Pattern<'data>,
        time: &Pattern<'data>,
    ) -> Result<Pattern<'data>, PatternError> {
        let size = date.items.len() + time.items.len();
        let mut result = Vec::with_capacity(self.items.len() + size);

        for item in self.items.iter() {
            match item {
                GenericPatternItem::Placeholder(1) => {
                    result.extend(date.items.iter());
                }
                GenericPatternItem::Placeholder(0) => {
                    result.extend(time.items.iter());
                }
                GenericPatternItem::Placeholder(idx) => {
                    let idx = char::from_digit(idx as u32, 10)
                        .expect("Failed to convert placeholder idx to char");
                    return Err(PatternError::UnknownSubstitution(idx));
                }
                GenericPatternItem::Literal(ch) => result.push(PatternItem::Literal(ch)),
            }
        }

        Ok(Pattern::from(result))
    }
}

impl From<reference::GenericPattern> for GenericPattern<'_> {
    fn from(input: reference::GenericPattern) -> Self {
        Self {
            items: ZeroVec::Owned(input.items.into_iter().map(|i| i.as_unaligned()).collect()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::pattern::{reference, runtime};

    #[test]
    fn test_runtime_generic_pattern_combine() {
        let pattern = reference::GenericPattern::from_bytes("{1} 'at' {0}")
            .expect("Failed to parse a generic pattern.");
        let pattern: runtime::GenericPattern = pattern.into();

        let date =
            reference::Pattern::from_bytes("Y/m/d").expect("Failed to parse a date pattern.");
        let date: runtime::Pattern = date.into();

        let time =
            reference::Pattern::from_bytes("HH:mm").expect("Failed to parse a time pattern.");
        let time: runtime::Pattern = time.into();

        let pattern = pattern
            .combined(&date, &time)
            .expect("Failed to combine date and time.");
        let pattern = reference::Pattern::from(pattern.items.to_vec());

        assert_eq!(pattern.to_string(), "Y/m/d 'at' HH:mm");
    }
}

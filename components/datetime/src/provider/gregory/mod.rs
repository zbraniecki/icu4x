// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(missing_docs)] // TODO(#686) - Add missing docs.

mod skeletons;
mod symbols;

use crate::pattern;
use alloc::borrow::Cow;
use icu_provider::yoke::{self, *};
pub use skeletons::*;
pub use symbols::*;

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct DatePatternsV1<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub date: patterns::LengthPatternsV1<'data>,

    /// These patterns are common uses of time formatting, broken down by the length of the
    /// pattern. Users can override the hour cycle with a preference, so there are two
    /// pattern groups stored here. Note that the pattern will contain either h11 or h12.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub time_h11_h12: patterns::LengthPatternsV1<'data>,

    /// These patterns are common uses of time formatting, broken down by the length of the
    /// pattern. Users can override the hour cycle with a preference, so there are two
    /// pattern groups stored here. Note that the pattern will contain either h23 or h24.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub time_h23_h24: patterns::LengthPatternsV1<'data>,

    /// By default a locale will prefer one hour cycle type over another.
    pub preferred_hour_cycle: pattern::CoarseHourCycle,

    /// Patterns used to combine date and time length patterns into full date_time patterns.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub length_combinations: patterns::GenericLengthPatternsV1<'data>,
}

pub mod patterns {
    use super::*;
    use crate::pattern::runtime::{self, GenericPattern};
    use crate::pattern::{self, runtime::Pattern};
    use core::convert::TryFrom;
    use icu_provider::yoke::{self, Yokeable, ZeroCopyFrom};
    use icu_provider::DataMarker;

    #[derive(Debug, PartialEq, Clone, Default, Yokeable, ZeroCopyFrom)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct LengthPatternsV1<'data> {
        #[cfg_attr(feature = "provider_serde", serde(borrow))]
        pub full: Pattern<'data>,
        #[cfg_attr(feature = "provider_serde", serde(borrow))]
        pub long: Pattern<'data>,
        #[cfg_attr(feature = "provider_serde", serde(borrow))]
        pub medium: Pattern<'data>,
        #[cfg_attr(feature = "provider_serde", serde(borrow))]
        pub short: Pattern<'data>,
    }

    #[derive(Debug, PartialEq, Clone, Default, Yokeable, ZeroCopyFrom)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct GenericLengthPatternsV1<'data> {
        #[cfg_attr(feature = "provider_serde", serde(borrow))]
        pub full: GenericPattern<'data>,
        #[cfg_attr(feature = "provider_serde", serde(borrow))]
        pub long: GenericPattern<'data>,
        #[cfg_attr(feature = "provider_serde", serde(borrow))]
        pub medium: GenericPattern<'data>,
        #[cfg_attr(feature = "provider_serde", serde(borrow))]
        pub short: GenericPattern<'data>,
    }

    /// This struct is a public wrapper around the internal [`Pattern`] struct. This allows
    /// access to the serialization and deserialization capabilities, without exposing the
    /// internals of the pattern machinery.
    ///
    /// The [`Pattern`] is an "exotic type" in the serialization process, and handles its own
    /// custom serialization practices.
    #[icu_provider::data_struct]
    #[derive(Debug, PartialEq, Clone, Default)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct PatternV1<'data>(
        #[cfg_attr(feature = "provider_serde", serde(borrow))] pub Pattern<'data>,
    );

    impl<'data> From<Pattern<'data>> for PatternV1<'data> {
        fn from(pattern: Pattern<'data>) -> Self {
            Self(pattern)
        }
    }

    /// Helper struct used to allow for projection of `DataPayload<DatePatternsV1>` to
    /// `DataPayload<PatternV1>`.
    pub struct PatternFromPatternsV1Marker;

    impl<'data> DataMarker<'data> for PatternFromPatternsV1Marker {
        type Yokeable = PatternV1<'static>;
        type Cart = DatePatternsV1<'data>;
    }
}

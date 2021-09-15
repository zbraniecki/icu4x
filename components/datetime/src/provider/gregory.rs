// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(missing_docs)] // TODO(#686) - Add missing docs.

use crate::pattern;
use alloc::borrow::Cow;
use icu_provider::yoke::{self, *};

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(cloning_zcf)]
pub struct DateSymbolsV1 {
    pub months: months::ContextsV1,

    pub weekdays: weekdays::ContextsV1,

    pub day_periods: day_periods::ContextsV1,
}

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(cloning_zcf)]
pub struct DatePatternsV1<'data> {
    #[serde(borrow)]
    pub date: patterns::LengthPatternsV1<'data>,

    /// These patterns are common uses of time formatting, broken down by the length of the
    /// pattern. Users can override the hour cycle with a preference, so there are two
    /// pattern groups stored here. Note that the pattern will contain either h11 or h12.
    #[serde(borrow)]
    pub time_h11_h12: patterns::LengthPatternsV1<'data>,

    /// These patterns are common uses of time formatting, broken down by the length of the
    /// pattern. Users can override the hour cycle with a preference, so there are two
    /// pattern groups stored here. Note that the pattern will contain either h23 or h24.
    #[serde(borrow)]
    pub time_h23_h24: patterns::LengthPatternsV1<'data>,

    /// By default a locale will prefer one hour cycle type over another.
    pub preferred_hour_cycle: pattern::CoarseHourCycle,

    #[serde(borrow)]
    pub datetime: patterns::DateTimeFormatsV1<'data>,
}
macro_rules! symbols {
        ($name: ident, $expr: ty) => {
            pub mod $name {
                use super::*;

                #[derive(Debug, PartialEq, Clone, Default)]
                #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
                pub struct SymbolsV1(pub $expr);

                symbols!();
            }
        };
        ($name: ident { $($tokens: tt)* }) => {
            symbols!($name { $($tokens)* } -> ());
        };
        ($name: ident { $element: ident: Option<$ty: ty>, $($tokens: tt)+ } -> ($($members:tt)*)) => {
            symbols!($name { $($tokens)* } -> (
                $($members)*
                pub $element: Option<$ty>,
            ));
        };
        ($name: ident { $element: ident: $ty: ty, $($tokens: tt)+ } -> ($($members:tt)*)) => {
            symbols!($name { $($tokens)* } -> (
                $($members)*
                pub $element: $ty,
            ));
        };
        ($name: ident { $element: ident: Option<$ty: ty> $(,)? } -> ($($members:tt)*)) => {
            symbols!($name { } -> (
                $($members)*
                pub $element: Option<$ty>,
            ));
        };
        ($name: ident { $element: ident: $ty: ty $(,)? } -> ($($members:tt)*)) => {
            symbols!($name { } -> (
                $($members)*
                pub $element: $ty,
            ));
        };
        ($name: ident { } -> ($($members: tt)*)) => {
            pub mod $name {
                use super::*;

                #[derive(Debug, PartialEq, Clone, Default, Yokeable, ZeroCopyFrom)]
                #[yoke(cloning_zcf)]
                #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
                pub struct SymbolsV1 {
                    $($members)*
                }
                symbols!();
            }
        };
        () => {
            // UTS 35 specifies that `format` widths are mandatory
            // except of `short`.
            #[derive(Debug, PartialEq, Clone, Default, Yokeable, ZeroCopyFrom)]
            #[yoke(cloning_zcf)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct FormatWidthsV1 {
                pub abbreviated: SymbolsV1,
                pub narrow: SymbolsV1,
                pub short: Option<SymbolsV1>,
                pub wide: SymbolsV1,
            }

            // UTS 35 specifies that `stand_alone` widths are optional
            #[derive(Debug, PartialEq, Clone, Default, Yokeable, ZeroCopyFrom)]
            #[yoke(cloning_zcf)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct StandAloneWidthsV1 {
                pub abbreviated: Option<SymbolsV1>,
                pub narrow: Option<SymbolsV1>,
                pub short: Option<SymbolsV1>,
                pub wide: Option<SymbolsV1>,
            }

            #[derive(Debug, PartialEq, Clone, Default, Yokeable, ZeroCopyFrom)]
            #[yoke(cloning_zcf)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct ContextsV1 {
                pub format: FormatWidthsV1,
                pub stand_alone: Option<StandAloneWidthsV1>,
            }
        };
    }

symbols!(months, [Cow<'static, str>; 12]);

symbols!(weekdays, [Cow<'static, str>; 7]);

symbols!(
    day_periods {
        am: Cow<'static, str>,
        pm: Cow<'static, str>,
        noon: Option<Cow<'static, str>>,
        midnight: Option<Cow<'static, str>>,
    }
);

pub mod patterns {
    use super::*;
    use crate::{
        pattern::{self, generic::GenericPattern, Pattern},
        skeleton::{Skeleton, SkeletonError},
    };
    use core::convert::TryFrom;
    use icu_provider::DataMarker;
    use litemap::LiteMap;

    #[derive(Debug, PartialEq, Clone)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct LengthPatternsV1<'data> {
        #[serde(borrow)]
        pub full: PatternV1<'data>,
        #[serde(borrow)]
        pub long: PatternV1<'data>,
        #[serde(borrow)]
        pub medium: PatternV1<'data>,
        #[serde(borrow)]
        pub short: PatternV1<'data>,
    }

    /// This struct is a public wrapper around the internal [`Pattern`] struct. This allows
    /// access to the serialization and deserialization capabilities, without exposing the
    /// internals of the pattern machinery.
    ///
    /// The [`Pattern`] is an "exotic type" in the serialization process, and handles its own
    /// custom serialization practices.
    #[icu_provider::data_struct]
    #[derive(Debug, PartialEq, Clone)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct PatternV1<'data>(
        #[serde(borrow)] pub zerovec::ZeroVec<'data, crate::pattern::PatternItem>,
    );

    impl<'data> From<Pattern<'data>> for PatternV1<'data> {
        fn from(pattern: Pattern<'data>) -> Self {
            Self(pattern.items)
        }
    }

    impl TryFrom<&str> for PatternV1<'_> {
        type Error = pattern::Error;

        fn try_from(pattern_string: &str) -> Result<Self, Self::Error> {
            let pattern = Pattern::from_bytes(pattern_string);
            match pattern {
                Ok(pattern) => Ok(Self::from(pattern)),
                Err(err) => Err(err),
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct LengthGenericPatternsV1<'data> {
        #[serde(borrow)]
        pub full: GenericPatternV1<'data>,
        #[serde(borrow)]
        pub long: GenericPatternV1<'data>,
        #[serde(borrow)]
        pub medium: GenericPatternV1<'data>,
        #[serde(borrow)]
        pub short: GenericPatternV1<'data>,
    }

    #[derive(Debug, PartialEq, Clone)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct GenericPatternV1<'data>(
        #[serde(borrow)] pub zerovec::ZeroVec<'data, crate::pattern::generic::GenericPatternItem>,
    );

    impl<'data> From<GenericPattern<'data>> for GenericPatternV1<'data> {
        fn from(pattern: GenericPattern<'data>) -> Self {
            Self(pattern.items)
        }
    }

    impl TryFrom<&str> for GenericPatternV1<'_> {
        type Error = pattern::Error;

        fn try_from(pattern_string: &str) -> Result<Self, Self::Error> {
            let pattern = GenericPattern::from_bytes(pattern_string);
            match pattern {
                Ok(pattern) => Ok(Self::from(pattern)),
                Err(err) => Err(err),
            }
        }
    }

    /// Helper struct used to allow for projection of `DataPayload<DatePatternsV1>` to
    /// `DataPayload<PatternV1>`.
    pub struct PatternFromPatternsV1Marker;

    impl<'data> DataMarker<'data> for PatternFromPatternsV1Marker {
        type Yokeable = PatternV1<'static>;
        type Cart = DatePatternsV1<'data>;
    }

    /// This struct is a public wrapper around the internal `Skeleton` struct. This allows
    /// access to the serialization and deserialization capabilities, without exposing the
    /// internals of the skeleton machinery.
    ///
    /// The `Skeleton` is an "exotic type" in the serialization process, and handles its own
    /// custom serialization practices.
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct SkeletonV1(pub Skeleton);

    impl TryFrom<&str> for SkeletonV1 {
        type Error = SkeletonError;

        fn try_from(skeleton_string: &str) -> Result<Self, Self::Error> {
            match Skeleton::try_from(skeleton_string) {
                Ok(skeleton) => Ok(Self(skeleton)),
                Err(err) => Err(err),
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Default)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct SkeletonsV1<'data>(#[serde(borrow)] pub LiteMap<SkeletonV1, PatternV1<'data>>);

    #[derive(Debug, PartialEq, Clone)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct DateTimeFormatsV1<'data> {
        #[serde(borrow)]
        pub length_patterns: LengthGenericPatternsV1<'data>,
        #[serde(borrow)]
        pub skeletons: SkeletonsV1<'data>,
    }
}

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::borrow::Cow;

#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct DatesV1<'s> {
    #[serde(borrow)]
    pub symbols: DateSymbolsV1<'s>,

    #[serde(borrow)]
    pub patterns: PatternsV1<'s>,
}

#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct DateSymbolsV1<'s> {
    #[serde(borrow)]
    pub months: months::ContextsV1<'s>,

    #[serde(borrow)]
    pub weekdays: weekdays::ContextsV1<'s>,

    #[serde(borrow)]
    pub day_periods: day_periods::ContextsV1<'s>,
}

#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct PatternsV1<'s> {
    #[serde(borrow)]
    pub date: patterns::StylePatternsV1<'s>,

    #[serde(borrow)]
    pub time: patterns::StylePatternsV1<'s>,

    #[serde(borrow)]
    pub date_time: patterns::DateTimeFormatsV1<'s>,
}

macro_rules! symbols {
        ($name: ident, $expr: ty) => {
            pub mod $name {
                use super::*;

                #[derive(Debug, PartialEq, Clone, Default)]
                #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
                pub struct SymbolsV1<'s>(pub $expr);

                symbols!();
            }
        };
        ($name: ident { $($tokens: tt)* }) => {
            symbols!($name { $($tokens)* } -> ());
        };
        ($name: ident { $element: ident: Option<$ty: ty>, $($tokens: tt)+ } -> ($($members:tt)*)) => {
            symbols!($name { $($tokens)* } -> (
                $($members)*
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
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
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
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

                #[derive(Debug, PartialEq, Clone, Default)]
                #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
                pub struct SymbolsV1<'s> {
                    $($members)*
                }
                symbols!();
            }
        };
        () => {
            // UTS 35 specifies that `format` widths are mandatory
            // except of `short`.
            #[derive(Debug, PartialEq, Clone, Default)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct FormatWidthsV1<'s> {
                #[serde(borrow)]
                pub abbreviated: SymbolsV1<'s>,
                #[serde(borrow)]
                pub narrow: SymbolsV1<'s>,
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                #[serde(borrow)]
                pub short: Option<SymbolsV1<'s>>,
                #[serde(borrow)]
                pub wide: SymbolsV1<'s>,
            }

            // UTS 35 specifies that `stand_alone` widths are optional
            #[derive(Debug, PartialEq, Clone, Default)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct StandAloneWidthsV1<'s> {
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                #[serde(borrow)]
                pub abbreviated: Option<SymbolsV1<'s>>,
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                #[serde(borrow)]
                pub narrow: Option<SymbolsV1<'s>>,
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                #[serde(borrow)]
                pub short: Option<SymbolsV1<'s>>,
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                #[serde(borrow)]
                pub wide: Option<SymbolsV1<'s>>,
            }

            #[derive(Debug, PartialEq, Clone, Default)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct ContextsV1<'s> {
                #[serde(borrow)]
                pub format: FormatWidthsV1<'s>,
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                #[serde(borrow)]
                pub stand_alone: Option<StandAloneWidthsV1<'s>>,
            }
        };
    }

symbols!(months, [Cow<'s, str>; 12]);

symbols!(weekdays, [Cow<'s, str>; 7]);

symbols!(
    day_periods {
        am: Cow<'s, str>,
        pm: Cow<'s, str>,
        noon: Option<Cow<'s, str>>,
        midnight: Option<Cow<'s, str>>,
    }
);

pub mod patterns {
    use super::*;
    use crate::{
        pattern::{self, Pattern},
        skeleton::{Skeleton, SkeletonError},
    };
    use litemap::LiteMap;
    use std::convert::TryFrom;

    #[derive(Debug, PartialEq, Clone, Default)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct StylePatternsV1<'s> {
        #[serde(borrow)]
        pub full: Cow<'s, str>,
        #[serde(borrow)]
        pub long: Cow<'s, str>,
        #[serde(borrow)]
        pub medium: Cow<'s, str>,
        #[serde(borrow)]
        pub short: Cow<'s, str>,
    }

    /// This struct is a public wrapper around the internal Pattern struct. This allows
    /// access to the serialization and deserialization capabilities, without exposing the
    /// internals of the pattern machinery.
    ///
    /// The Pattern is an "exotic type" in the serialization process, and handles its own
    /// custom serialization practices.
    // #[derive(Debug, PartialEq, Clone, Default)]
    // #[cfg_attr(
    //     feature = "provider_serde",
    //     derive(serde::Serialize, serde::Deserialize)
    // )]
    // pub struct PatternV1<'s>(
    //     // #[cfg_attr(
    //     //     feature = "provider_serde",
    //     //     serde(borrow)
    //     // )]
    //     pub Pattern<'s>
    // );

    // impl<'s> From<Pattern<'s>> for PatternV1<'s> {
    //     fn from(pattern: Pattern<'s>) -> Self {
    //         Self(pattern)
    //     }
    // }

    // impl<'s> TryFrom<&'s str> for PatternV1<'s> {
    //     type Error = pattern::Error;

    //     fn try_from(pattern_string: &'s str) -> Result<Self, Self::Error> {
    //         let pattern = Pattern::from_bytes(pattern_string);
    //         match pattern {
    //             Ok(pattern) => Ok(PatternV1::from(pattern)),
    //             Err(err) => Err(err),
    //         }
    //     }
    // }

    /// This struct is a public wrapper around the internal Skeleton struct. This allows
    /// access to the serialization and deserialization capabilities, without exposing the
    /// internals of the skeleton machinery.
    ///
    /// The Skeleton is an "exotic type" in the serialization process, and handles its own
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
                Ok(skeleton) => Ok(SkeletonV1(skeleton)),
                Err(err) => Err(err),
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Default)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct SkeletonsV1<'s>(#[serde(borrow)] pub LiteMap<SkeletonV1, Pattern<'s>>);

    #[derive(Debug, PartialEq, Clone, Default)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct DateTimeFormatsV1<'s> {
        #[serde(borrow)]
        pub style_patterns: StylePatternsV1<'s>,
        #[serde(borrow)]
        pub skeletons: SkeletonsV1<'s>,
    }
}

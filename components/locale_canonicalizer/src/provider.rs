// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::LanguageIdentifier;

pub mod key {
    use icu_provider::{resource_key, ResourceKey};
    pub const LIKELY_SUBTAGS_V1: ResourceKey = resource_key!(likelysubtags, "likelysubtags", 1);
}

#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct LikelySubtagsV1 {
    pub language_script: Vec<(u32, u32, LanguageIdentifier)>,
    pub language_region: Vec<(u32, u32, LanguageIdentifier)>,
    pub language: Vec<(u32, LanguageIdentifier)>,
    pub script_region: Vec<(u32, u32, LanguageIdentifier)>,
    pub script: Vec<(u32, LanguageIdentifier)>,
    pub region: Vec<(u32, LanguageIdentifier)>,
    pub und: LanguageIdentifier,
}

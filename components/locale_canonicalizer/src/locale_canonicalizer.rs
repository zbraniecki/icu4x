// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::provider::*;
use icu_locid::{LanguageIdentifier, Locale};
use icu_provider::prelude::*;
use std::borrow::Cow;

/// CanonicalizationResult is used to track the result of a canonicalization
/// operation that potentially modifies its argument in place.
#[derive(Debug, PartialEq)]
pub enum CanonicalizationResult {
    Modified,
    Unmodified,
}

pub struct LocaleCanonicalizer<'a> {
    likely_subtags: Cow<'a, LikelySubtagsV1>,
}

#[inline]
fn update_locale(
    entry: &LanguageIdentifier,
    locale: &mut Locale,
) -> CanonicalizationResult {
    if locale.language.is_empty() {
        locale.language = entry.language;
    }
    locale.script = locale.script.or(entry.script);
    locale.region = locale.region.or(entry.region);
    CanonicalizationResult::Modified
}

macro_rules! maximize_locale {
    ( $locale:ident, $table:expr, $key:expr ) => {{
        if let Ok(index) = $table.binary_search_by_key(&&$key, |(i1, _)| i1) {
            let entry = &$table[index].1;
            return update_locale(entry, $locale);
        }
    }};
    ( $locale:ident, $table:expr, $key1:expr, $key2:expr ) => {{
        if let Ok(index) = $table.binary_search_by_key(&(&$key1, &$key2), |(i1, i2, _)| (i1, i2)) {
            let entry = &$table[index].2;
            return update_locale(entry, $locale);
        }
    }};
}

impl LocaleCanonicalizer<'_> {
    /// A constructor which takes a DataProvider and creates a
    /// LocaleCanonicalizer.
    pub fn new<'d>(
        provider: &(impl DataProvider<'d, LikelySubtagsV1> + ?Sized),
    ) -> Result<LocaleCanonicalizer<'d>, DataError> {
        let payload: Cow<LikelySubtagsV1> = provider
            .load_payload(&DataRequest::from(key::LIKELY_SUBTAGS_V1))?
            .take_payload()?;

        Ok(LocaleCanonicalizer {
            likely_subtags: payload,
        })
    }

    /// The maximize method potentially updates a passed in locale in place
    /// depending up the results of running the 'Add Likely Subtags' algorithm
    /// from https://www.unicode.org/reports/tr35/#Likely_Subtags.
    ///
    /// If the result of running the algorithm would result in a new locale, the
    /// locale argument is updated in place to match the result, and the method
    /// returns `CanonicalizationResult::Modified`. Otherwise, the method
    /// returns `CanonicalizationResult::Unmodified` and the locale argument is
    /// unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "provider_serde")] {
    /// use icu_locale_canonicalizer::{CanonicalizationResult, LocaleCanonicalizer};
    /// use icu_locid::Locale;
    ///
    /// let provider = icu_testdata::get_provider();
    /// let lc = LocaleCanonicalizer::new(&provider).unwrap();
    ///
    /// let mut locale : Locale = "en-US".parse().unwrap();
    /// assert_eq!(lc.maximize(&mut locale), CanonicalizationResult::Modified);
    /// assert_eq!(locale.to_string(), "en-Latn-US");
    ///
    /// let mut locale : Locale = "en-Latn-DE".parse().unwrap();
    /// assert_eq!(lc.maximize(&mut locale), CanonicalizationResult::Unmodified);
    /// assert_eq!(locale.to_string(), "en-Latn-DE");
    /// # } // feature = "provider_serde"
    /// ```
    pub fn maximize(&self, locale: &mut Locale) -> CanonicalizationResult {
        if !locale.language.is_empty() && locale.script.is_some() && locale.region.is_some() {
            return CanonicalizationResult::Unmodified;
        }

        if let Some(language) = locale.language.into_raw() {
            if let Some(region) = &locale.region {
                maximize_locale!(
                    locale,
                    self.likely_subtags.language_region,
                    language,
                    region.into_raw()
                );
            }
            if let Some(script) = &locale.script {
                maximize_locale!(
                    locale,
                    self.likely_subtags.language_script,
                    language,
                    script.into_raw()
                );
            }

            maximize_locale!(locale, self.likely_subtags.language, language);
        } else if let Some(script) = &locale.script {
            if let Some(region) = &locale.region {
                maximize_locale!(
                    locale,
                    self.likely_subtags.script_region,
                    script.into_raw(),
                    region.into_raw()
                );
            }

            maximize_locale!(locale, self.likely_subtags.script, script.into_raw());
        } else if let Some(region) = &locale.region {
            maximize_locale!(locale, self.likely_subtags.region, region.into_raw());
        }
        update_locale(&self.likely_subtags.und, locale)
    }

    /// This returns a new Locale that is the result of running the
    /// 'Remove Likely Subtags' algorithm from
    /// https://www.unicode.org/reports/tr35/#Likely_Subtags.
    ///
    /// If the result of running the algorithm would result in a new locale, the
    /// locale argument is updated in place to match the result, and the method
    /// returns `CanonicalizationResult::Modified`. Otherwise, the method
    /// returns `CanonicalizationResult::Unmodified` and the locale argument is
    /// unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "provider_serde")] {
    /// use icu_locale_canonicalizer::{CanonicalizationResult, LocaleCanonicalizer};
    /// use icu_locid::Locale;
    ///
    /// let provider = icu_testdata::get_provider();
    /// let lc = LocaleCanonicalizer::new(&provider).unwrap();
    ///
    /// let mut locale : Locale = "en-Latn-US".parse().unwrap();
    /// assert_eq!(lc.minimize(&mut locale), CanonicalizationResult::Modified);
    /// assert_eq!(locale.to_string(), "en");
    ///
    /// let mut locale : Locale = "en".parse().unwrap();
    /// assert_eq!(lc.minimize(&mut locale), CanonicalizationResult::Unmodified);
    /// assert_eq!(locale.to_string(), "en");
    /// # } // feature = "provider_serde"
    /// ```
    pub fn minimize(&self, locale: &mut Locale) -> CanonicalizationResult {
        let mut max = locale.clone();
        self.maximize(&mut max);
        max.variants.clear();
        let mut trial = max.clone();

        trial.script = None;
        trial.region = None;
        self.maximize(&mut trial);
        if trial == max {
            if locale.script.is_some() || locale.script.is_some() {
                locale.script = None;
                locale.region = None;
                return CanonicalizationResult::Modified;
            } else {
                return CanonicalizationResult::Unmodified;
            }
        }

        trial.script = None;
        trial.region = max.region;
        self.maximize(&mut trial);
        if trial == max {
            if locale.script.is_some() || locale.region != max.region {
                locale.script = None;
                locale.region = max.region;
                return CanonicalizationResult::Modified;
            } else {
                return CanonicalizationResult::Unmodified;
            }
        }

        trial.script = max.script;
        trial.region = None;
        self.maximize(&mut trial);
        if trial == max {
            if locale.script != max.script || locale.region.is_some() {
                locale.script = max.script;
                locale.region = None;
                return CanonicalizationResult::Modified;
            } else {
                return CanonicalizationResult::Unmodified;
            }
        }

        if locale.script != max.script || locale.region != max.region {
            locale.script = max.script;
            locale.region = max.region;
            CanonicalizationResult::Modified
        } else {
            CanonicalizationResult::Unmodified
        }
    }
}

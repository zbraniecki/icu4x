use icu_locale::{
    provider::{LocaleLikelySubtagsLanguageV1, LocaleLikelySubtagsScriptRegionV1},
    LanguageIdentifier, Locale, LocaleExpander, TransformResult,
};
use icu_provider::{DataError, DataProvider};

use crate::{
    config::{NegotiateConfig, NegotiationStrategy},
    MatchedLocale, WeightedLocale,
};

pub struct LocaleNegotiator {
    expander: LocaleExpander,
}

impl LocaleNegotiator {
    #[cfg(feature = "compiled_data")]
    pub const fn new_common() -> Self {
        Self {
            expander: LocaleExpander::new_common(),
        }
    }

    pub fn try_new_common_unstable<P>(provider: &P) -> Result<Self, DataError>
    where
        P: DataProvider<LocaleLikelySubtagsLanguageV1>
            + DataProvider<LocaleLikelySubtagsScriptRegionV1>
            + ?Sized,
    {
        Ok(Self {
            expander: LocaleExpander::try_new_common_unstable(provider)?,
        })
    }

    pub fn filter_and_sort(
        &self,
        available: &[Locale],
        requested: &[WeightedLocale],
        config: NegotiateConfig,
    ) -> Vec<MatchedLocale> {
        let mut supported_locales = vec![];

        let mut available_locales: Vec<&Locale> = available.iter().collect();

        macro_rules! test_strategy {
            ($req:ident, $self_as_range:expr, $other_as_range:expr) => {{
                let mut match_found = false;
                available_locales.retain(|locale| {
                    if config.strategy != NegotiationStrategy::Filtering && match_found {
                        return true;
                    }

                    if langid_matches(&locale.id, &$req.locale.id, $self_as_range, $other_as_range)
                    {
                        match_found = true;
                        supported_locales.push(*locale);
                        return false;
                    }
                    true
                });

                if match_found {
                    match config.strategy {
                        NegotiationStrategy::Filtering => {}
                        NegotiationStrategy::Matching => continue,
                        NegotiationStrategy::BestMatch => break,
                    }
                }
            }};
        }

        for req in requested {
            // 1) Try to find a simple (case-insensitive) string match for the request.
            test_strategy!(req, false, false);

            // 2) Try to match against the available locales treated as ranges.
            test_strategy!(req, true, false);

            // Per Unicode TR35, 4.4 Locale Matching, we don't add likely subtags to
            // requested locales, so we'll skip it from the rest of the steps.
            if req.locale.id.language.is_unknown() {
                continue;
            }

            let mut req = req.clone();

            // 3) Try to match against a maximized version of the requested locale
            if self.expander.maximize(&mut req.locale.id) == TransformResult::Modified {
                test_strategy!(req, true, false);
            }

            // 4) Try to match against a variant as a range
            req.locale.id.variants.clear();
            test_strategy!(req, true, true);

            // 5) Try to match against the likely subtag without region
            req.locale.id.region = None;
            if self.expander.maximize(&mut req.locale.id) == TransformResult::Modified {
                test_strategy!(req, true, false);
            }

            // 6) Try to match against a region as a range
            req.locale.id.region = None;
            test_strategy!(req, true, true);
        }

        let mut result: Vec<_> = supported_locales
            .into_iter()
            .map(|locale| MatchedLocale {
                locale: locale.clone(),
                score: 1.0,
                matched_against: locale.clone(),
            })
            .collect();

        if result.is_empty() {
            if let Some(locale) = &config.default_locale {
                result.push(MatchedLocale {
                    locale: locale.clone(),
                    score: 0.0,
                    matched_against: requested.first().unwrap().locale.clone(),
                })
            }
        }
        result
    }
}

fn subtag_matches<P: PartialEq>(
    subtag1: &Option<P>,
    subtag2: &Option<P>,
    as_range1: bool,
    as_range2: bool,
) -> bool {
    (as_range1 && subtag1.is_none()) || (as_range2 && subtag2.is_none()) || subtag1 == subtag2
}

#[inline(always)]
fn langid_matches(
    lid1: &LanguageIdentifier,
    lid2: &LanguageIdentifier,
    range1: bool,
    range2: bool,
) -> bool {
    ((range1 && lid1.language.is_unknown())
        || (range2 && lid2.language.is_unknown())
        || lid1.language == lid2.language)
        && subtag_matches(&lid1.script, &lid2.script, range1, range2)
        && subtag_matches(&lid1.region, &lid2.region, range1, range2)
        && ((range1 && lid1.variants.is_empty())
            || (range2 && lid2.variants.is_empty())
            || lid1.variants == lid2.variants)
}

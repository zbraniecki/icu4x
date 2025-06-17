pub mod config;
mod negotiatior;
mod requested;
mod weighted_locale;

use icu_locale::Locale;
pub use negotiatior::LocaleNegotiator;
pub use requested::LocaleListBuilder;
pub use weighted_locale::WeightedLocale;

// struct DesiredLocaleMatch {
//     /// The original desired locale
//     desired: Locale,
//     /// The original weight/priority of this desired locale
//     weight: f32,
//     /// The best available locale that matches this desired locale (if any)
//     best_match: Option<Locale>,
//     /// The match score (0.0-1.0)
//     score: f32,
// }

#[derive(Debug, Clone)]
pub struct MatchedLocale {
    /// The matched available locale
    pub locale: Locale,
    /// The match score (0.0-1.0, higher is better)
    pub score: f32,
    /// The desired locale this matched against
    pub matched_against: Locale,
}

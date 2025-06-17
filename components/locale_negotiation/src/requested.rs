use crate::weighted_locale::WeightedLocale;
use icu_locale::Locale;

/// A builder for creating weighted locale lists from various inputs
#[derive(Debug, Default)]
pub struct LocaleListBuilder {
    locales: Vec<WeightedLocale>,
}

impl LocaleListBuilder {
    /// Create a new empty builder
    pub fn new() -> Self {
        Self {
            locales: Vec::new(),
        }
    }

    /// Add a locale with explicit weight
    pub fn add(&mut self, locale: Locale, weight: f32) -> &mut Self {
        self.locales.push(WeightedLocale::new(locale, weight));
        self
    }

    /// Add a locale from string with explicit weight
    pub fn add_str(&mut self, s: &str, weight: f32) -> Result<&mut Self, icu_locale::ParseError> {
        self.locales.push(WeightedLocale::from_str(s, weight)?);
        Ok(self)
    }

    /// Add locales from Accept-Language header format (e.g., "en-US,en;q=0.9,fr;q=0.8")
    pub fn from_accept_language_str(
        &mut self,
        _header: &str,
    ) -> Result<&mut Self, Box<dyn std::error::Error>> {
        // Implementation for Accept-Language header parsing
        todo!();

        #[allow(unreachable_code)]
        Ok(self)
    }

    /// Build the final list of weighted locales
    pub fn build(&self) -> Vec<WeightedLocale> {
        // Sort by weight in descending order
        let mut result = self.locales.clone();
        result.sort_by(|a, b| {
            b.weight
                .partial_cmp(&a.weight)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        result
    }
}

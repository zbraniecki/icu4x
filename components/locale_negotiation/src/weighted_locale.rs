use icu_locale::Locale;

#[derive(Debug, Clone)]
pub struct WeightedLocale {
    pub(crate) locale: Locale,
    pub(crate) weight: f32,
}

impl WeightedLocale {
    pub fn new(locale: Locale, weight: f32) -> Self {
        Self { locale, weight }
    }

    pub fn from_str(input: &str, weight: f32) -> Result<Self, icu_locale::ParseError> {
        Ok(Self {
            locale: input.parse()?,
            weight,
        })
    }
}

impl PartialEq<Locale> for WeightedLocale {
    fn eq(&self, other: &Locale) -> bool {
        self.locale == *other
    }
}

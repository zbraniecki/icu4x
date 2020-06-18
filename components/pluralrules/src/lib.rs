pub mod data;
pub mod operands;
pub mod rules;

use icu_locale::LanguageIdentifier;
use operands::PluralOperands;
use rules::ast;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum PluralRuleType {
    Ordinal,
    Cardinal,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum PluralCategory {
    Zero,
    One,
    Two,
    Few,
    Many,
    Other,
}

impl PluralCategory {
    pub fn iter() -> &'static [Self] {
        &[
            Self::Zero,
            Self::One,
            Self::Two,
            Self::Few,
            Self::Many,
            Self::Other,
        ]
    }
}

pub struct PluralRules {
    locale: LanguageIdentifier,
    rules: Box<[(PluralCategory, ast::Condition)]>,
}

impl PluralRules {
    pub fn try_new(locale: LanguageIdentifier, type_: PluralRuleType) -> Result<Self, ()> {
        let rules = data::get_rules(&locale, type_);
        Ok(Self { locale, rules })
    }

    pub fn select<I: Into<PluralOperands>>(&self, input: I) -> PluralCategory {
        let operands: PluralOperands = input.into();
        rules::resolver::select(&self.rules, &operands)
    }

    pub fn get_locale(&self) -> &LanguageIdentifier {
        &self.locale
    }

    pub fn my_test(&self) -> bool {
        true
    }
}

mod data;
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

#[derive(Debug, Eq, PartialEq)]
pub enum PluralCategory {
    Zero,
    One,
    Two,
    Few,
    Many,
    Other,
}

pub struct PluralRules {
    locale: LanguageIdentifier,
    rule: ast::Condition,
}

impl PluralRules {
    pub fn try_new(locale: LanguageIdentifier, type_: PluralRuleType) -> Result<Self, ()> {
        let rule = data::get_rule(&locale, type_);
        Ok(Self { locale, rule })
    }

    pub fn select<I: Into<PluralOperands>>(&self, input: I) -> PluralCategory {
        let operands: PluralOperands = input.into();
        if rules::resolver::matches(&self.rule, &operands) {
            PluralCategory::One
        } else {
            PluralCategory::Other
        }
    }

    pub fn get_locale(&self) -> &LanguageIdentifier {
        &self.locale
    }
}

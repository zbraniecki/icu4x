use crate::rules::ast;
use crate::rules::parser::Parser;
use crate::PluralRuleType;
use icu_locale::LanguageIdentifier;

pub fn get_rule(locale: &LanguageIdentifier, type_: PluralRuleType) -> ast::Condition {
    let input = match (locale.language.as_str(), type_) {
        ("en", PluralRuleType::Cardinal) => b"i = 1",
        _ => unimplemented!(),
    };
    let parser = Parser::new(input);
    parser.parse().expect("Failed to parse")
}

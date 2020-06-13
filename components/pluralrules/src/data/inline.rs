use crate::rules::ast;
use crate::rules::parser::Parser;
use crate::PluralCategory;
use crate::PluralRuleType;
use icu_locale::LanguageIdentifier;

const EN_DATA_CARDINAL: &[(PluralCategory, &str)] = &[
    (PluralCategory::One, "i = 1 and v = 0 @integer 1"),
    (PluralCategory::Other, " @integer 0, 2~16, 100, 1000, 10000, 100000, 1000000, … @decimal 0.0~1.5, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …"),
];

const PL_DATA_CARDINAL: &[(PluralCategory, &str)] = &[
    (PluralCategory::One, "i = 1 and v = 0"),
    (PluralCategory::Few, "v = 0 and i % 10 = 2..4 and i % 100 != 12..14"),
    (PluralCategory::Many, "v = 0 and i != 1 and i % 10 = 0..1 or v = 0 and i % 10 = 5..9 or v = 0 and i % 100 = 12..14"),
    (PluralCategory::Other, ""),
];

pub fn get_rules(
    locale: &LanguageIdentifier,
    _type_: PluralRuleType,
) -> Box<[(PluralCategory, ast::Condition)]> {
    let mut rules = vec![];
    let data = match locale.to_string().as_str() {
        "pl" => PL_DATA_CARDINAL,
        "en" => EN_DATA_CARDINAL,
        _ => unimplemented!(),
    };

    for (category, input) in data {
        let p = Parser::new(input.as_bytes());
        let ast = p
            .parse()
            .expect(&format!("Failed to parse: {:#?}, {:#?}", locale, category));
        rules.push((*category, ast));
    }
    rules.into_boxed_slice()
}

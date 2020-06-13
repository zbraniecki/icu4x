use crate::data::json::Resource;
use crate::rules::ast;
use crate::rules::parser::Parser;
use crate::PluralCategory;
use crate::PluralRuleType;
use bincode;
use icu_locale::LanguageIdentifier;
use std::fs::File;
use std::io::Read;

static mut ORDINALS_STRING: Vec<u8> = vec![];
static mut CARDINALS_STRING: Vec<u8> = vec![];
static mut ORDINALS: Option<Resource> = None;
static mut CARDINALS: Option<Resource> = None;

pub fn get_resource(type_: PluralRuleType) -> &'static Resource<'static> {
    let rules = match type_ {
        PluralRuleType::Cardinal => unsafe { &mut ORDINALS },
        PluralRuleType::Ordinal => unsafe { &mut CARDINALS },
    };

    let s = match type_ {
        PluralRuleType::Cardinal => unsafe { &mut ORDINALS_STRING },
        PluralRuleType::Ordinal => unsafe { &mut CARDINALS_STRING },
    };

    if rules.is_none() {
        let path = match type_ {
            PluralRuleType::Cardinal => "./data/plurals.dat",
            PluralRuleType::Ordinal => "./data/ordinals.dat",
        };

        let mut fh = File::open(path).expect("Opening file failed");

        fh.read_to_end(s).expect("Failed to read");

        let res: Resource = bincode::deserialize(s).unwrap();
        *rules = Some(res);
    }

    rules.as_ref().unwrap()
}

pub fn get_rules(
    locale: &LanguageIdentifier,
    type_: PluralRuleType,
) -> Box<[(PluralCategory, ast::Condition)]> {
    let res = get_resource(type_);

    let mut result = vec![];

    let rules = match type_ {
        PluralRuleType::Cardinal => res.supplemental.plurals_type_cardinal.as_ref().unwrap(),
        PluralRuleType::Ordinal => res.supplemental.plurals_type_ordinal.as_ref().unwrap(),
    };

    let lang = locale.to_string();
    let lang_rules = rules.get(&lang.as_str()).unwrap();

    for category in PluralCategory::iter() {
        if category == &PluralCategory::Other {
            continue;
        }

        if let Some(input) = lang_rules.get(category) {
            let p = Parser::new(input.as_bytes());
            let ast = p
                .parse()
                .expect(&format!("Failed to parse: {:#?}, {:#?}", locale, category));
            result.push((*category, ast));
        }
    }
    result.into_boxed_slice()
}

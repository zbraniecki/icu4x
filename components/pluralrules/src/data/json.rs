use crate::rules::ast;
use crate::rules::parser::Parser;
use crate::PluralCategory;
use crate::PluralRuleType;
use icu_locale::LanguageIdentifier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Resource<'s> {
    #[serde(borrow)]
    pub supplemental: Supplemental<'s>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct PluralRules<'s> {
    #[serde(rename = "pluralRule-count-zero")]
    pub zero: Option<&'s str>,
    #[serde(rename = "pluralRule-count-one")]
    pub one: Option<&'s str>,
    #[serde(rename = "pluralRule-count-two")]
    pub two: Option<&'s str>,
    #[serde(rename = "pluralRule-count-few")]
    pub few: Option<&'s str>,
    #[serde(rename = "pluralRule-count-many")]
    pub many: Option<&'s str>,
}

impl<'s> PluralRules<'s> {
    pub fn get(&self, category: &PluralCategory) -> Option<&'s str> {
        match category {
            PluralCategory::Zero => self.zero,
            PluralCategory::One => self.one,
            PluralCategory::Two => self.two,
            PluralCategory::Few => self.few,
            PluralCategory::Many => self.many,
            PluralCategory::Other => None,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Supplemental<'s> {
    #[serde(rename = "plurals-type-cardinal")]
    #[serde(borrow)]
    pub plurals_type_cardinal: Option<HashMap<&'s str, PluralRules<'s>>>,
    #[serde(rename = "plurals-type-ordinal")]
    #[serde(borrow)]
    pub plurals_type_ordinal: Option<HashMap<&'s str, PluralRules<'s>>>,
}

static mut ORDINALS_STRING: String = String::new();
static mut CARDINALS_STRING: String = String::new();
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
            PluralRuleType::Cardinal => "./data/plurals.json",
            PluralRuleType::Ordinal => "./data/ordinals.json",
        };
        File::open(path).unwrap().read_to_string(s).unwrap();

        let res: Resource = serde_json::from_str(s).unwrap();
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
            let ast = p.parse().unwrap();
            result.push((*category, ast));
        }
    }
    result.into_boxed_slice()
}

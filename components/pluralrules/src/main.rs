use icu_locale::LanguageIdentifier;
use icu_pluralrules::rules::lexer::Lexer;
use icu_pluralrules::rules::parser::Parser;
use icu_pluralrules::rules::resolver::matches;
use icu_pluralrules::{PluralRuleType, PluralRules};

pub fn main() {
    let s = "   @decimal 0.0~1.5, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …";
    let l = Lexer::new(s.as_bytes());
    for token in l {
        println!("{:#?}", token);
    }

    let p = Parser::new(s.as_bytes());
    let ast = p.parse().unwrap();
    println!("{:#?}", &ast);

    println!("{:#?}", matches(&ast, &2u64.into()));

    let lang: LanguageIdentifier = "pl".parse().unwrap();
    let pr = PluralRules::try_new(lang, PluralRuleType::Cardinal).unwrap();
    println!("{:#?}", pr.select(5 as usize));
}

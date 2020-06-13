use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;
use std::convert::TryInto;

use cldr_pluralrules_parser::{parse_plural_condition, parse_plural_rule};
use icu_locale::LanguageIdentifier;
use icu_pluralrules::operands::PluralOperands;
use icu_pluralrules::rules::lexer::Lexer;
use icu_pluralrules::rules::parser::Parser;
use icu_pluralrules::{PluralCategory, PluralRuleType, PluralRules};

const SAMPLES: &[isize] = &[
    1, 2, 3, 4, 5, 25, 134, 910293019, 12, 1412, -12, 15, 2931, 31231, 3123, 13231, 91, 0, 231, -2,
    -45, 33, 728, 2, 291, 24, 479, 291, 778, 919, 93,
];

const PL_DATA: &[(PluralCategory, &'static str)] = &[
    (PluralCategory::One, "i = 1 and v = 0"),
    (PluralCategory::Few, "v = 0 and i % 10 = 2..4 and i % 100 != 12..14"),
    (PluralCategory::Many, "v = 0 and i != 1 and i % 10 = 0..1 or v = 0 and i % 10 = 5..9 or v = 0 and i % 100 = 12..14"),
    (PluralCategory::Other, ""),
];

fn plural_rules(c: &mut Criterion) {
    c.bench_function("lex", |b| {
        b.iter(|| {
            for (_, s) in PL_DATA {
                let lexer = Lexer::new(black_box(s.as_bytes()));
                let _ = lexer.count();
            }
        })
    });
    c.bench_function("parse", |b| {
        b.iter(|| {
            for (_, s) in PL_DATA {
                let parser = Parser::new(black_box(s.as_bytes()));
                let _ = parser.parse();
            }
        })
    });
    c.bench_function("parse_old", |b| {
        b.iter(|| {
            for (_, s) in PL_DATA {
                let _ = parse_plural_condition(black_box(s)).expect("Parsing succeeded");
            }
        })
    });
    c.bench_function("select", |b| {
        let loc: LanguageIdentifier = "pl".parse().unwrap();
        let pr = PluralRules::try_new(loc, PluralRuleType::Cardinal).unwrap();
        b.iter(|| {
            for s in SAMPLES {
                let op: PluralOperands = (*s).try_into().unwrap();
                let _ = pr.select(op);
            }
        })
    });

    let langs = &["uk", "de", "sk", "ar", "fr", "it", "en", "cs", "es", "zh"];
    let langs: Vec<LanguageIdentifier> = langs
        .iter()
        .map(|l| l.parse().expect("Parsing failed"))
        .collect();

    c.bench_with_input(
        BenchmarkId::new("construct", langs.len()),
        &langs,
        |b, langs| {
            b.iter(|| {
                for lang in langs {
                    PluralRules::try_new(lang.clone(), PluralRuleType::Ordinal).unwrap();
                    PluralRules::try_new(lang.clone(), PluralRuleType::Cardinal).unwrap();
                }
            });
        },
    );
}

criterion_group!(benches, plural_rules,);
criterion_main!(benches);

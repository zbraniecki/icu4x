use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use cldr_pluralrules_parser::{parse_plural_condition, parse_plural_rule};
use icu_pluralrules::rules::lexer::Lexer;
use icu_pluralrules::rules::parser::Parser;
use icu_pluralrules::rules::resolver::matches;

// let langs = &["uk", "de", "sk", "ar", "fr", "it", "en", "cs", "es", "zh"];
// const SAMPLES: &[isize] = &[
//     1, 2, 3, 4, 5, 25, 134, 910293019, 12, 1412, -12, 15, 2931, 31231, 3123, 13231, 91, 0, 231,
//     -2, -45, 33, 728, 2, 291, 24, 479, 291, 778, 919, 93,
// ];

const STRINGS: &[&str] = &[
    "i = 1",
    "n % 10 = 1 and n % 100 != 11",
    "n % 10 = 2..4 and n % 100 != 12..14",
    "n = 0,1 or i = 0 and f = 1",
];

fn plural_rules(c: &mut Criterion) {
    c.bench_function("lex", |b| {
        b.iter(|| {
            for s in STRINGS {
                let lexer = Lexer::new(black_box(s.as_bytes()));
                let _ = lexer.collect::<Vec<_>>();
            }
        })
    });
    c.bench_function("parse", |b| {
        b.iter(|| {
            for s in STRINGS {
                let parser = Parser::new(black_box(s.as_bytes()));
                let _ = parser.parse();
            }
        })
    });
    c.bench_function("parse_old", |b| {
        b.iter(|| {
            for s in STRINGS {
                let _ = parse_plural_rule(black_box(s)).expect("Parsing succeeded");
            }
        })
    });
    c.bench_function("matches", |b| {
        b.iter(|| {
            for s in STRINGS {
                let parser = Parser::new(black_box(s.as_bytes()));
                let ast = parser.parse().unwrap();
                let _ = matches(&ast, &1u64.into());
            }
        })
    });
    // let langs = &["uk", "de", "sk", "ar", "fr", "it", "en", "cs", "es", "zh"];
    // let langs: Vec<LanguageIdentifier> = langs
    //     .iter()
    //     .map(|l| l.parse().expect("Parsing failed"))
    //     .collect();
    //
    // c.bench_with_input(
    //     BenchmarkId::new("construct", langs.len()),
    //     &langs,
    //     |b, langs| {
    //         b.iter(|| {
    //             for lang in langs {
    //                 PluralRules::create(lang.clone(), PluralRuleType::ORDINAL).unwrap();
    //                 PluralRules::create(lang.clone(), PluralRuleType::CARDINAL).unwrap();
    //             }
    //         });
    //     },
    // );
    //
    // let samples = &[
    //     1, 2, 3, 4, 5, 25, 134, 910293019, 12, 1412, -12, 15, 2931, 31231, 3123, 13231, 91, 0, 231,
    //     -2, -45, 33, 728, 2, 291, 24, 479, 291, 778, 919, 93,
    // ];
    //
    // let langid_pl = langid!("pl");
    // let ipr = PluralRules::create(langid_pl.clone(), PluralRuleType::CARDINAL).unwrap();
    //
    // c.bench_with_input(
    //     BenchmarkId::new("select", samples.len()),
    //     samples,
    //     |b, samples| {
    //         b.iter(|| {
    //             for value in samples {
    //                 ipr.select(*value).unwrap();
    //             }
    //         });
    //     },
    // );
    //
    // c.bench_function("total", |b| {
    //     b.iter(|| {
    //         let ipr = PluralRules::create(langid_pl.clone(), PluralRuleType::CARDINAL).unwrap();
    //         ipr.select(1).unwrap();
    //         ipr.select(2).unwrap();
    //         ipr.select(3).unwrap();
    //         ipr.select(4).unwrap();
    //         ipr.select(5).unwrap();
    //         ipr.select(25).unwrap();
    //         ipr.select(134).unwrap();
    //         ipr.select(5090).unwrap();
    //         ipr.select(910293019).unwrap();
    //         ipr.select(5.2).unwrap();
    //         ipr.select(-0.2).unwrap();
    //         ipr.select("12.06").unwrap();
    //     })
    // });
}

criterion_group!(benches, plural_rules,);
criterion_main!(benches);

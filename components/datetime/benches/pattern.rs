// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
mod fixtures;

use criterion::{criterion_group, criterion_main, Criterion};

use icu_datetime::pattern::{Pattern, PatternItem};
use icu_datetime::pattern::Parser;

fn pattern_benches(c: &mut Criterion) {
    let patterns: Vec<String> = fixtures::get_patterns_fixture().unwrap().0;

    {
        let mut group = c.benchmark_group("pattern");

        group.bench_function("parse", |b| {
            b.iter(|| {
                for input in &patterns {
                    let _ = Pattern::from_bytes(input).unwrap();
                }
            })
        });

        let samples = vec![
            ("Foo {0} and {1}", vec!["Hello", "World"], "Foo Hello and World"),
            ("Foo {1} and {0}", vec!["Hello", "World"], "Foo World and Hello"),
            ("{0}, {1} and {2}", vec!["Start", "Middle", "End"], "Start, Middle and End"),
            // ("{0} 'at' {1}", vec!["Hello", "World"]),
        ];

        group.bench_function("parse_placeholder", |b| {
            b.iter(|| {
                for sample in &samples {
                    let parser = Parser::new(sample.0);
                    let replacements = sample.1.iter().map(|v| {
                        Pattern(vec![PatternItem::Literal(v.to_string())])
                    }).collect();
                    let result = parser.parse_placeholders(replacements).unwrap();
                    let mut s = String::new();
                    for item in result {
                        if let PatternItem::Literal(v) = item {
                            s.push_str(&v);
                        }
                    }
                    assert_eq!(s, sample.2);
                }
            })
        });

        group.finish();
    }
}

criterion_group!(benches, pattern_benches,);
criterion_main!(benches);

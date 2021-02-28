// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_simple_formatter::*;

#[derive(Debug)]
struct Token;

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn parser(c: &mut Criterion) {
    let samples = vec![
        ("Foo {0} and {1}", vec![vec!["Hello"], vec!["World"]]),
        ("Foo {1} and {0}", vec![vec!["Hello"], vec!["World"]]),
        (
            "{0}, {1} and {2}",
            vec![vec!["Start"], vec!["Middle"], vec!["End"]],
        ),
        // ("{start}, {midde} and {end}", vec!["Start", "Middle", "End"]),
        ("{0} 'at' {1}", vec![vec!["Hello"], vec!["World"]]),
    ];

    c.bench_function("parser/idx", |b| {
        b.iter(|| {
            for sample in &samples {
                let _ = parse::<usize>(sample.0).count();
            }
        })
    });

    c.bench_function("interpolate/idx", |b| {
        b.iter(|| {
            for sample in &samples {
                let iter = parse(sample.0);

                let replacements: Vec<Vec<Element<Token>>> = sample
                    .1
                    .iter()
                    .map(|r| r.iter().map(|t| Element::Literal(t)).collect())
                    .collect();
                let i = interpolate(iter, replacements).count();
            }
        })
    });
}

criterion_group!(benches, parser,);
criterion_main!(benches);

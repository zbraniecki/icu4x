// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_simple_formatter::*;

fn parser(c: &mut Criterion) {
    let samples = vec![
        ("Foo {0} and {1}", vec!["Hello", "World"]),
        ("Foo {1} and {0}", vec!["Hello", "World"]),
        ("{start}, {midde} and {end}", vec!["Start", "Middle", "End"]),
        // ("{0} 'at' {1}", vec!["Hello", "World"]),
    ];

    c.bench_function("parser/idx", |b| {
        b.iter(|| {
            for sample in &samples {
                let parser = Parser::new();
                let _ = parser.parse(sample.0);
            }
        })
    });

    c.bench_function("format/idx", |b| {
        b.iter(|| {
            for sample in &samples {
                let parser = Parser::new();
                let elements = parser.parse(sample.0);
                let mut result = String::new();
                write_format(&mut result, &elements, &sample.1);
            }
        })
    });
}

criterion_group!(benches, parser,);
criterion_main!(benches);

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;
mod helpers;

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use icu_locale_core::subtags::{Language, Region, Script, Variant};
use icu_locale_core::ParseError;

macro_rules! subtag_bench {
    ($c:expr, $name:expr, $subtag:ident, $data:expr) => {
        $c.bench_function(&format!("subtags/{}/utf8/parse", $name), |b| {
            b.iter(|| {
                for s in &$data.valid {
                    let _: $subtag = $subtag::try_from_utf8(black_box(s).as_bytes()).unwrap();
                }
                for s in &$data.invalid {
                    let _: ParseError =
                        $subtag::try_from_utf8(black_box(s).as_bytes()).unwrap_err();
                }
            })
        });

        let data_valid_utf16: Vec<Vec<u16>> = $data
            .valid
            .iter()
            .map(|s| s.encode_utf16().collect())
            .collect();
        let data_invalid_utf16: Vec<Vec<u16>> = $data
            .invalid
            .iter()
            .map(|s| s.encode_utf16().collect())
            .collect();
        $c.bench_function(&format!("subtags/{}/utf16/parse", $name), |b| {
            b.iter(|| {
                for s in &data_valid_utf16 {
                    let _: $subtag = $subtag::try_from_utf16(black_box(s)).unwrap();
                }
                for s in &data_invalid_utf16 {
                    let _: ParseError = $subtag::try_from_utf16(black_box(s)).unwrap_err();
                }
            })
        });
    };
}

fn subtags_bench(c: &mut Criterion) {
    let data = serde_json::from_str::<fixtures::Subtags>(include_str!("fixtures/subtags.json"))
        .expect("Failed to read a fixture");

    subtag_bench!(c, "language", Language, data.language);
    subtag_bench!(c, "script", Script, data.script);
    subtag_bench!(c, "region", Region, data.region);
    subtag_bench!(c, "variant", Variant, data.variant);
}

criterion_group!(benches, subtags_bench,);
criterion_main!(benches);

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_datetime::{
    fields::{Field, FieldLength, FieldSymbol, Month, Year},
    pattern::{Pattern, PatternItem, ZVPattern},
};
use postcard::from_bytes;
use zerovec::ZeroVec;

fn pattern_benches(c: &mut Criterion) {
    let data = (
        // Postcard
        &[
            0b0000_0011,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0001,
            0b0000_0000,
            0b0000_0001,
            0b0000_0001,
            0b0000_0001,
            0b0110_0001,
        ],
        // ZeroVec
        &[
            0b0000_0000,
            0b0000_0000,
            0b0000_0001,
            0b0000_0000,
            0b0000_0001,
            0b0000_0010,
            0b1000_0000,
            0b0000_0000,
            0b0110_0001,
        ],
        &[
            PatternItem::Field(Field {
                symbol: FieldSymbol::Year(Year::Calendar),
                length: FieldLength::One,
            }),
            PatternItem::Field(Field {
                symbol: FieldSymbol::Month(Month::Short),
                length: FieldLength::TwoDigit,
            }),
            PatternItem::Literal('a'),
        ],
    );
    let mut group = c.benchmark_group("criterion/load");
    group.bench_function("from_items", |b| {
        b.iter(|| {
            let _ = Pattern(black_box(data).2.to_vec());
        })
    });
    group.bench_function("from_postcard", |b| {
        b.iter(|| {
            let _: Pattern = from_bytes(black_box(data).0).unwrap();
        })
    });
    group.bench_function("from_zerovec", |b| {
        b.iter(|| {
            let _ = ZVPattern(ZeroVec::try_from_bytes(black_box(data).1).unwrap());
        })
    });
    group.finish();
}

criterion_group!(benches, pattern_benches,);
criterion_main!(benches);

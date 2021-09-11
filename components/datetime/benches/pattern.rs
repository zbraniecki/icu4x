// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_datetime::fixtures::{PatternList, PatternStringList, ZVPatternList};
use postcard::from_bytes;
use std::fs;
use zerovec::VarZeroVec;

fn pattern_benches(c: &mut Criterion) {
    let pattern_string_list_json = fs::read("./data/pattern_strings.json").unwrap();
    let pattern_list_json = fs::read("./data/pattern_structs.json").unwrap();
    let pattern_strings_postcard = fs::read("./data/pattern_strings.postcard").unwrap();
    let pattern_structs_postcard = fs::read("./data/pattern_structs.postcard").unwrap();
    let pattern_structs_zv = fs::read("./data/pattern_structs.zv").unwrap();

    let mut group = c.benchmark_group("criterion/load");
    group.bench_function("from_strings_json", |b| {
        b.iter(|| {
            let json: PatternStringList =
                serde_json::from_slice(&pattern_string_list_json).unwrap();
            let _ = PatternList::from(black_box(&json));
        })
    });
    group.bench_function("from_structs_json", |b| {
        b.iter(|| {
            let _: PatternList = serde_json::from_slice(&pattern_list_json).unwrap();
        })
    });
    group.bench_function("from_strings_postcard", |b| {
        b.iter(|| {
            let result: PatternStringList = from_bytes(&pattern_strings_postcard).unwrap();
            let _ = PatternList::from(black_box(&result));
        })
    });
    group.bench_function("from_structs_postcard", |b| {
        b.iter(|| {
            let _: PatternList = from_bytes(&pattern_structs_postcard).unwrap();
        })
    });
    group.bench_function("from_structs_zerovec", |b| {
        b.iter(|| {
            let zvpl: ZVPatternList<'_> =
                ZVPatternList(VarZeroVec::try_from_bytes(&pattern_structs_zv).unwrap());
            let _: PatternList = zvpl.into();
        })
    });
    group.finish();
}

criterion_group!(benches, pattern_benches,);
criterion_main!(benches);

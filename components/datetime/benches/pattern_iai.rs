// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use iai::black_box;
use icu_datetime::fixtures::{PatternList, PatternStringList, ZVPatternList};
use postcard::from_bytes;
use std::fs;
use zerovec::VarZeroVec;

fn iai_pattern_strings_json() {
    let pattern_string_list_json = fs::read("./data/pattern_strings.json").unwrap();
    let json: PatternStringList = serde_json::from_slice(&pattern_string_list_json).unwrap();
    let _ = PatternList::from(black_box(&json));
}

fn iai_pattern_structs_json() {
    let pattern_list_json = fs::read("./data/pattern_structs.json").unwrap();
    let _: PatternList = serde_json::from_slice(&pattern_list_json).unwrap();
}

fn iai_pattern_strings_postcard() {
    let pattern_strings_postcard = fs::read("./data/pattern_strings.postcard").unwrap();
    let result: PatternStringList = from_bytes(&pattern_strings_postcard).unwrap();
    let _ = PatternList::from(black_box(&result));
}

fn iai_pattern_structs_postcard() {
    let pattern_structs_postcard = fs::read("./data/pattern_structs.postcard").unwrap();
    let _: PatternList = from_bytes(&pattern_structs_postcard).unwrap();
}

fn iai_pattern_structs_zv() {
    let pattern_structs_zv = fs::read("./data/pattern_structs.zv").unwrap();
    let zvpl: ZVPatternList<'_> =
        ZVPatternList(VarZeroVec::try_from_bytes(&pattern_structs_zv).unwrap());
    let _: PatternList = zvpl.into();
}

iai::main!(
    iai_pattern_strings_json,
    iai_pattern_structs_json,
    iai_pattern_strings_postcard,
    iai_pattern_structs_postcard,
    iai_pattern_structs_zv
);

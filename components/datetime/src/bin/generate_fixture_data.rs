// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::fixtures::{
    get_pattern_string_list, PatternList, PatternStringList, ZVPatternList,
};
use postcard::{from_bytes, to_allocvec};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let pattern_string_list = get_pattern_string_list().unwrap();

    let mut file = File::create("./data/pattern_strings.postcard").unwrap();
    let bytes: Vec<u8> = to_allocvec(&pattern_string_list).unwrap();
    file.write_all(&bytes).unwrap();

    {
        let result: PatternStringList = from_bytes(&bytes).unwrap();
        assert_eq!(pattern_string_list, result);
    }

    let patterns = PatternList::from(&pattern_string_list);
    serde_json::to_writer(
        &File::create("./data/pattern_structs.json").unwrap(),
        &patterns,
    )
    .unwrap();
    let bytes: Vec<u8> = to_allocvec(&patterns).unwrap();
    let mut file = File::create("./data/pattern_structs.postcard").unwrap();
    file.write_all(&bytes).unwrap();

    {
        let result: PatternList = from_bytes(&bytes).unwrap();
        assert_eq!(patterns, result);
    }

    let zv_pattern_list = ZVPatternList::from(&patterns);
    let bytes: Vec<u8> = to_allocvec(&zv_pattern_list).unwrap();
    let mut file = File::create("./data/pattern_zv.postcard").unwrap();
    file.write_all(&bytes).unwrap();

    {
        let result: ZVPatternList = from_bytes(&bytes).unwrap();
        assert_eq!(zv_pattern_list, result);
        let result2 = PatternList::from(result);
        assert_eq!(patterns, result2);
    }
}

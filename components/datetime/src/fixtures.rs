// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::pattern::{Pattern, ZVPattern};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use zerovec::ZeroVec;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatternList(pub Vec<Pattern>);

impl From<&PatternStringList> for PatternList {
    fn from(input: &PatternStringList) -> Self {
        Self(
            input
                .0
                .iter()
                .map(|s| Pattern::from_bytes(s).unwrap())
                .collect(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZVPatternList(pub Vec<Vec<u8>>);

impl From<&PatternList> for ZVPatternList {
    fn from(input: &PatternList) -> Self {
        Self(
            input
                .0
                .iter()
                .map(|p| {
                    let zv_pattern: ZVPattern = p.into();
                    zv_pattern.0.as_bytes().to_vec()
                })
                .collect(),
        )
    }
}

impl From<ZVPatternList> for PatternList {
    fn from(input: ZVPatternList) -> Self {
        Self(
            input
                .0
                .into_iter()
                .map(|bytes| {
                    let zv_pattern = ZVPattern(ZeroVec::try_from_bytes(&bytes).unwrap());
                    zv_pattern.into()
                })
                .collect(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatternStringList(pub Vec<String>);

#[allow(dead_code)]
pub fn get_pattern_string_list() -> std::io::Result<PatternStringList> {
    let file = File::open("./data/pattern_strings.json")?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

#[allow(dead_code)]
pub fn get_pattern_list() -> std::io::Result<PatternList> {
    let file = File::open("./data/pattern_structs.json")?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

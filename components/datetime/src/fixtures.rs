// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::pattern::{PatternItem, VecPattern, ZVPattern};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use zerovec::{ule::AsULE, VarZeroVec, ZeroVec};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatternList(pub Vec<VecPattern>);

impl From<&PatternStringList> for PatternList {
    fn from(input: &PatternStringList) -> Self {
        Self(
            input
                .0
                .iter()
                .map(|s| VecPattern::from_bytes(s).unwrap())
                .collect(),
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ZVPatternList<'data>(pub VarZeroVec<'data, ZeroVec<'static, PatternItem>>);

impl<'data> From<&PatternList> for ZVPatternList<'_> {
    fn from(patterns: &PatternList) -> Self {
        let zv_patterns: Vec<ZeroVec<'static, PatternItem>> = patterns
            .0
            .iter()
            .map(|p| ZVPattern::from(p).items)
            .collect();
        Self(zv_patterns.into())
    }
}

impl From<ZVPatternList<'_>> for PatternList {
    fn from(vzv: ZVPatternList<'_>) -> Self {
        Self(
            vzv.0
                .iter()
                .map(|zv| VecPattern {
                    items: zv
                        .iter()
                        .map(|epi| PatternItem::from_unaligned(epi))
                        .collect(),
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

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatternsFixture(pub Vec<String>);

use std::fs::File;
use std::io::BufReader;

#[allow(dead_code)]
pub fn get_patterns_fixture() -> std::io::Result<PatternsFixture> {
    let file = File::open("./data/patterns.json")?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
}

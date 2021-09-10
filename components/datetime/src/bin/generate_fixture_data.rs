// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::{
    fields::{Field, FieldLength, FieldSymbol, Month, Year},
    fixtures::get_patterns_fixture,
    pattern::{Pattern, PatternItem},
};
use postcard::{from_bytes, to_allocvec};

fn main() {
    let data = get_patterns_fixture().unwrap();
    for s in &data.0 {
        let pattern = Pattern::from_bytes(s).unwrap();
        println!("{:?}", pattern);
    }
}

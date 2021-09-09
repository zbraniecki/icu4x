// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::{
    fields::{Field, FieldLength, FieldSymbol, Month, Year},
    pattern::{Pattern, PatternItem},
};
use postcard::{from_bytes, to_allocvec};

fn main() {
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
    let pattern = Pattern(data.2.to_vec());
    println!("{:#?}", pattern);
    let bytes: Vec<u8> = to_allocvec(&pattern).unwrap();
    for b in &bytes {
        println!("{:#010b}", b);
    }
    let result: Pattern = from_bytes(&bytes).unwrap();
    assert_eq!(pattern, result);
    assert_eq!(bytes, data.0);
}

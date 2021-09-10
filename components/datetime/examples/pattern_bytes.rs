// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::{
    fields::{Day, Field, FieldLength, FieldSymbol, Month, Year},
    pattern::{Pattern, PatternItem, ZVPattern},
};
use postcard::{from_bytes, to_allocvec};
use zerovec::ZeroVec;

fn main() {
    let data = (
        // Postcard
        &[
            0b00000101, 0b00000000, 0b00000010, 0b00000000, 0b00000001, 0b00000001, 0b00000001,
            0b00101111, 0b00000000, 0b00000001, 0b00000000, 0b00000001, 0b00000001, 0b00000001,
            0b00101111, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ],
        // ZeroVec
        &[
            0b0000_0000,
            0b0000_0010,
            0b0000_0010,
            0b1000_0000,
            0b0000_0000,
            0b0010_1111,
            0b0000_0000,
            0b0000_0001,
            0b0000_0010,
            0b1000_0000,
            0b0000_0000,
            0b0010_1111,
            0b0000_0000,
            0b0000_0000,
            0b0000_0001,
        ],
        &[
            PatternItem::Field(Field {
                symbol: FieldSymbol::Day(Day::DayOfMonth),
                length: FieldLength::TwoDigit,
            }),
            PatternItem::Literal('/'),
            PatternItem::Field(Field {
                symbol: FieldSymbol::Month(Month::Format),
                length: FieldLength::TwoDigit,
            }),
            PatternItem::Literal('/'),
            PatternItem::Field(Field {
                symbol: FieldSymbol::Year(Year::Calendar),
                length: FieldLength::One,
            }),
        ],
    );
    let pattern: Pattern = data.2.to_vec().into();
    // println!("{:#?}", pattern);
    // let bytes: Vec<u8> = to_allocvec(&pattern).unwrap();
    // for b in &bytes {
    //     println!("{:#010b}", b);
    // }
    // let result: Pattern = from_bytes(&bytes).unwrap();
    // assert_eq!(pattern, result);
    // assert_eq!(bytes, data.0);
    let zv_pattern = ZVPattern::from(&pattern);
    let bytes = zv_pattern.0.as_bytes();
    // for b in bytes.iter() {
    //     println!("{:#010b}", b);
    // }
    assert_eq!(bytes, data.1);
    let new_zv_pattern = ZVPattern(ZeroVec::try_from_bytes(bytes).unwrap());
    assert_eq!(new_zv_pattern, zv_pattern);
}

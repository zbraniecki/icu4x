// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use iai::black_box;
use icu_datetime::{
    fields::{Field, FieldLength, FieldSymbol, Month, Year},
    pattern::{Pattern, PatternItem, ZVPattern},
};
use postcard::from_bytes;
use zerovec::ZeroVec;

fn iai_pattern_items() {
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
    let _ = Pattern(black_box(data).2.to_vec());
}

fn iai_pattern_postcard() {
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
    let _: Pattern = from_bytes(black_box(data).0).unwrap();
}

fn iai_pattern_zv() {
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
    let _ = ZVPattern(ZeroVec::try_from_bytes(black_box(data).1).unwrap());
}

iai::main!(iai_pattern_items, iai_pattern_postcard, iai_pattern_zv);

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use iai::black_box;

enum RangeOrValue {
    Range(u32, u32),
    Value(u32),
}

impl RangeOrValue {
    #[inline]
    fn as_unaligned(&self) -> [u8; 8] {
        match self {
            Self::Range(start, end) => {
                let start_bytes = start.to_le_bytes();
                let end_bytes = end.to_le_bytes();
                [
                    start_bytes[0],
                    start_bytes[1],
                    start_bytes[2],
                    start_bytes[3],
                    end_bytes[0],
                    end_bytes[1],
                    end_bytes[2],
                    end_bytes[3],
                ]
            }
            Self::Value(idx) => {
                let bytes = idx.to_le_bytes();
                [
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[0], bytes[1], bytes[2], bytes[3],
                ]
            }
        }
    }

    #[inline]
    fn as_unaligned2(&self) -> [u8; 8] {
        match self {
            Self::Range(start, end) => {
                let start_bytes = start.to_le_bytes();
                let end_bytes = end.to_le_bytes();
                let mut c = [0; 8];
                c[..4].copy_from_slice(&start_bytes);
                c[4..].copy_from_slice(&end_bytes);
                c
            }
            Self::Value(idx) => {
                let bytes = idx.to_le_bytes();
                let mut c = [0; 8];
                c[..4].copy_from_slice(&bytes);
                c[4..].copy_from_slice(&bytes);
                c
            }
        }
    }
}

fn iai_rule_1() {
    // let v = RangeOrValue::Value(126);
    let v = RangeOrValue::Range(126, 256);
    let _ = black_box(v.as_unaligned());
}

fn iai_rule_2() {
    // let v = RangeOrValue::Value(126);
    let v = RangeOrValue::Range(126, 256);
    let _ = black_box(v.as_unaligned2());
}

iai::main!(iai_rule_1, iai_rule_2,);

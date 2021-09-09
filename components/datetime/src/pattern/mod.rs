use super::fields;
use std::convert::TryFrom;
use zerovec::{
    ule::{AsULE, ULE},
    ZeroVec,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EncodedPatternItem(pub [u8; 3]);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PatternItem {
    Field(fields::Field),
    Literal(char),
}

impl EncodedPatternItem {
    pub fn bytes_in_range(value: (&u8, &u8, &u8)) -> bool {
        let first_bit = 0b1000_0000 & value.0 != 0;
        match first_bit {
            false => fields::Field::bytes_in_range((value.0, value.1), value.2),
            true => {
                let first_cleared = value.0 ^ 0b1000_0000;
                let u = u32::from_le_bytes([*value.2, *value.1, first_cleared, 0x00]);
                char::try_from(u).is_ok()
            }
        }
    }
}

impl ULE for EncodedPatternItem {
    type Error = ();

    fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error> {
        let mut chunks = bytes.chunks_exact(3);

        if !chunks.all(|c| Self::bytes_in_range((&c[0], &c[1], &c[2])))
            || !chunks.remainder().is_empty()
        {
            return Err(());
        }
        let data = bytes.as_ptr();
        let len = bytes.len() / 3;
        Ok(unsafe { std::slice::from_raw_parts(data as *const Self, len) })
    }

    fn as_byte_slice(slice: &[Self]) -> &[u8] {
        panic!();
        // let data = slice.as_ptr();
        // let len = slice.len();
        // unsafe { std::slice::from_raw_parts(data as *const u8, len) }
    }
}

impl AsULE for PatternItem {
    type ULE = EncodedPatternItem;

    #[inline]
    fn as_unaligned(&self) -> Self::ULE {
        match self {
            Self::Field(field) => {
                EncodedPatternItem([0x00, u8::from(field.symbol), field.length as u8])
            }
            Self::Literal(ch) => EncodedPatternItem([0x00, 0x00, 0x00]),
        }
    }

    #[inline]
    fn from_unaligned(unaligned: &Self::ULE) -> Self {
        let value = unaligned.0;
        let first_bit = 0b1000_0000 & value[0] != 0;
        match first_bit {
            false => {
                let symbol = fields::FieldSymbol::from(value[1]);
                let length = fields::FieldLength::from(value[2]);
                let field = fields::Field { symbol, length };
                PatternItem::Field(field)
            }
            true => {
                let first_cleared = value[0] ^ 0b1000_0000;
                let u = u32::from_le_bytes([value[2], value[1], first_cleared, 0x00]);
                PatternItem::Literal(char::try_from(u).unwrap())
            }
        }
    }
}

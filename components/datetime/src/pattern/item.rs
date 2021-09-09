use crate::fields;
use std::convert::TryFrom;
use zerovec::ule::{AsULE, ULE};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EncodedPatternItem(pub [u8; 3]);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PatternItem {
    Field(fields::Field),
    Literal(char),
}

impl EncodedPatternItem {
    pub fn item_type_from_u8(byte: u8) -> bool {
        byte & 0b1000_0000 != 0
    }

    pub fn clear_type_in_u8(byte: u8) -> u8 {
        byte ^ 0b1000_0000
    }

    pub fn bytes_in_range(value: (&u8, &u8, &u8)) -> bool {
        match Self::item_type_from_u8(*value.0) {
            false => fields::Field::bytes_in_range((value.0, value.1), value.2),
            true => {
                let u = u32::from_le_bytes([
                    *value.2,
                    *value.1,
                    Self::clear_type_in_u8(*value.0),
                    0x00,
                ]);
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

    fn as_byte_slice(_slice: &[Self]) -> &[u8] {
        todo!();
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
            Self::Literal(ch) => {
                let u = *ch as u32;
                let bytes = u.to_be_bytes();
                EncodedPatternItem([
                    EncodedPatternItem::clear_type_in_u8(bytes[1]),
                    bytes[2],
                    bytes[3],
                ])
            }
        }
    }

    #[inline]
    fn from_unaligned(unaligned: &Self::ULE) -> Self {
        let value = unaligned.0;
        match EncodedPatternItem::item_type_from_u8(value[0]) {
            false => {
                let symbol = fields::FieldSymbol::from(value[1]);
                let length = fields::FieldLength::from(value[2]);
                let field = fields::Field { symbol, length };
                PatternItem::Field(field)
            }
            true => {
                let first_cleared = value[0] ^ 0b1000_0000;
                let u = u32::from_be_bytes([0x00, first_cleared, value[1], value[2]]);
                PatternItem::Literal(char::try_from(u).unwrap())
            }
        }
    }
}

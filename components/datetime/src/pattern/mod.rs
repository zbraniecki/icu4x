use zerovec::{
    ule::{AsULE, ULE},
    ZeroVec,
};
use std::convert::TryFrom;
use super::fields;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PatternItem {
    Field(fields::Field),
    Char(char),
}

impl PatternItem {
    pub fn bytes_in_range(value: (&u8, &u8, &u8)) -> bool {
        let first_bit = 0b1000_0000 & value.0 != 0;
        println!("first bit: {:#?}", first_bit);
        match first_bit {
            false => {
                fields::Field::bytes_in_range((value.0, value.1), value.2)
            },
            true => {
                panic!();
                // println!("{:#?}", value.0);
                // let first_cleared = value.0 & 0b1000_0000;
                // println!("{:#?}", first_cleared);
                // let u = u32::from_le_bytes([0x00, first_cleared , *value.1, *value.2]);
                // char::try_from(u).is_ok()
            },
        }
    }
}

impl ULE for PatternItem {
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

use std::convert::{TryFrom, TryInto};
use zerovec::ule::{AsULE, ULE};

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum FieldLength {
    One = 1,
    TwoDigit = 2,
    Abbreviated = 3,
    Wide = 4,
    Narrow = 5,
    Six = 6,
}

impl From<u8> for FieldLength {
    fn from(input: u8) -> Self {
        match input {
            1 => Self::One,
            2 => Self::TwoDigit,
            _ => panic!(),
        }
    }
}

impl FieldLength {
    pub fn u8_in_range(v: &u8) -> bool {
        (1..=6).contains(v)
    }
}

impl ULE for FieldLength {
    type Error = ();

    fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error> {
        if !bytes.iter().all(Self::u8_in_range) {
            return Err(());
        }
        let data = bytes.as_ptr();
        let len = bytes.len();
        Ok(unsafe { std::slice::from_raw_parts(data as *const Self, len) })
    }

    fn as_byte_slice(slice: &[Self]) -> &[u8] {
        let data = slice.as_ptr();
        let len = slice.len();
        unsafe { std::slice::from_raw_parts(data as *const u8, len) }
    }
}

impl AsULE for FieldLength {
    type ULE = Self;

    #[inline]
    fn as_unaligned(&self) -> Self::ULE {
        *self
    }

    #[inline]
    fn from_unaligned(unaligned: &Self::ULE) -> Self {
        *unaligned
    }
}

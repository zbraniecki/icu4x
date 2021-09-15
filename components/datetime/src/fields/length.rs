// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::{
    cmp::{Ord, PartialOrd},
    convert::TryFrom,
};
use displaydoc::Display;
use zerovec::ule::{AsULE, ULE};

#[derive(Display, Debug, PartialEq)]
pub enum LengthError {
    #[displaydoc("Invalid length")]
    InvalidLength,
}

#[cfg(feature = "std")]
impl std::error::Error for LengthError {}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[repr(u8)]
pub enum FieldLength {
    One = 1,
    TwoDigit = 2,
    Abbreviated = 3,
    Wide = 4,
    Narrow = 5,
    Six = 6,
}

impl FieldLength {
    pub fn u8_in_range(v: &u8) -> bool {
        (1..=6).contains(v)
    }
}

impl From<FieldLength> for u8 {
    fn from(length: FieldLength) -> Self {
        match length {
            FieldLength::One => 1,
            FieldLength::TwoDigit => 2,
            FieldLength::Abbreviated => 3,
            FieldLength::Wide => 4,
            FieldLength::Narrow => 5,
            FieldLength::Six => 6,
        }
    }
}

macro_rules! try_field_length {
    ($i:ty) => {
        impl TryFrom<$i> for FieldLength {
            type Error = LengthError;

            fn try_from(input: $i) -> Result<Self, Self::Error> {
                Ok(match input {
                    1 => Self::One,
                    2 => Self::TwoDigit,
                    3 => Self::Abbreviated,
                    4 => Self::Wide,
                    5 => Self::Narrow,
                    6 => Self::Six,
                    _ => return Err(LengthError::InvalidLength),
                })
            }
        }
    };
}

try_field_length!(u8);
try_field_length!(usize);

unsafe impl ULE for FieldLength {
    type Error = ();

    fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error> {
        if !bytes.iter().all(Self::u8_in_range) {
            return Err(());
        }
        let data = bytes.as_ptr();
        let len = bytes.len();
        Ok(unsafe { core::slice::from_raw_parts(data as *const Self, len) })
    }

    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &[Self] {
        let data = bytes.as_ptr();
        let len = bytes.len();
        core::slice::from_raw_parts(data as *const Self, len)
    }

    fn as_byte_slice(slice: &[Self]) -> &[u8] {
        let data = slice.as_ptr();
        let len = slice.len();
        unsafe { core::slice::from_raw_parts(data as *const u8, len) }
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

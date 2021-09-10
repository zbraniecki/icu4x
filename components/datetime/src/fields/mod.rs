mod length;
mod symbol;

pub use length::*;
pub use symbol::*;

use displaydoc::Display;
use std::convert::{TryFrom, TryInto};
use zerovec::ule::{AsULE, ULE};

#[derive(Display, Debug)]
pub enum Error {
    #[displaydoc("Field {0:?} is not a valid length")]
    InvalidLength(FieldSymbol),
}

impl std::error::Error for Error {}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Field {
    pub symbol: FieldSymbol,
    pub length: FieldLength,
}

impl Field {
    pub fn bytes_in_range(symbol: &u8, length: &u8) -> bool {
        FieldSymbol::kv_in_range(symbol) && FieldLength::u8_in_range(length)
    }
}

impl ULE for Field {
    type Error = ();

    fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error> {
        let mut chunks = bytes.chunks_exact(2);

        if !chunks.all(|c| Self::bytes_in_range(&c[0], &c[1])) || !chunks.remainder().is_empty() {
            return Err(());
        }
        let data = bytes.as_ptr();
        let len = bytes.len() / 2;
        Ok(unsafe { std::slice::from_raw_parts(data as *const Self, len) })
    }

    fn as_byte_slice(slice: &[Self]) -> &[u8] {
        let data = slice.as_ptr();
        let len = slice.len() * 2;
        unsafe { std::slice::from_raw_parts(data as *const u8, len) }
    }
}

impl AsULE for Field {
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

impl From<(FieldSymbol, FieldLength)> for Field {
    fn from(input: (FieldSymbol, FieldLength)) -> Self {
        Self {
            symbol: input.0,
            length: input.1,
        }
    }
}

impl TryFrom<(FieldSymbol, u8)> for Field {
    type Error = Error;
    fn try_from(input: (FieldSymbol, u8)) -> Result<Self, Self::Error> {
        let (symbol, length) = (
            input.0,
            input
                .1
                .try_into()
                .map_err(|_| Self::Error::InvalidLength(input.0))?,
        );
        Ok(Self { symbol, length })
    }
}

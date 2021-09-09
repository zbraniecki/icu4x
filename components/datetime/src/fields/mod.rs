mod length;
mod symbol;

pub use length::*;
pub use symbol::*;

use zerovec::ule::{AsULE, ULE};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Field {
    pub symbol: FieldSymbol,
    pub length: FieldLength,
}

impl Field {
    pub fn bytes_in_range(symbol: (&u8, &u8), length: &u8) -> bool {
        FieldSymbol::kv_in_range(symbol.0, symbol.1) && FieldLength::u8_in_range(length)
    }
}

impl ULE for Field {
    type Error = ();

    fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error> {
        let mut chunks = bytes.chunks_exact(3);

        if !chunks.all(|c| Self::bytes_in_range((&c[0], &c[1]), &c[2]))
            || !chunks.remainder().is_empty()
        {
            return Err(());
        }
        let data = bytes.as_ptr();
        let len = bytes.len() / 3;
        Ok(unsafe { std::slice::from_raw_parts(data as *const Self, len) })
    }

    fn as_byte_slice(slice: &[Self]) -> &[u8] {
        let data = slice.as_ptr();
        let len = slice.len() * 3;
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

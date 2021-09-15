// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::error::Error;
use super::parser::Parser;
use super::{Pattern, PatternItem};
#[cfg(feature = "provider_serde")]
use alloc::format;
use alloc::string::String;
#[cfg(feature = "provider_serde")]
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use core::{convert::TryFrom, fmt};
use core::{fmt::Write, iter::FromIterator};
use zerovec::ule::{AsULE, ULE};
use zerovec::ZeroVec;

#[cfg(feature = "provider_serde")]
use serde::{
    de,
    ser::{self, SerializeSeq},
    Deserialize, Deserializer, Serialize,
};

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct EncodedGenericPatternItem(pub [u8; 3]);

impl EncodedGenericPatternItem {
    pub fn is_literal_from_u8(byte: u8) -> bool {
        byte & 0b1000_0000 != 0
    }

    pub fn clear_type_in_u8(byte: u8) -> u8 {
        byte ^ 0b1000_0000
    }

    pub fn set_literal_in_u8(byte: u8) -> u8 {
        byte | 0b1000_0000
    }

    pub fn bytes_in_range(value: (&u8, &u8, &u8)) -> bool {
        match Self::is_literal_from_u8(*value.0) {
            false => *value.0 == 0 && *value.1 == 0,
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

unsafe impl ULE for EncodedGenericPatternItem {
    type Error = &'static str;

    fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error> {
        let mut chunks = bytes.chunks_exact(3);

        if !chunks.all(|c| Self::bytes_in_range((&c[0], &c[1], &c[2])))
            || !chunks.remainder().is_empty()
        {
            return Err("invalid bytes for EncodedPatternItem");
        }
        let data = bytes.as_ptr();
        let len = bytes.len() / 3;
        Ok(unsafe { core::slice::from_raw_parts(data as *const Self, len) })
    }

    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &[Self] {
        let data = bytes.as_ptr();
        let len = bytes.len() / 3;
        core::slice::from_raw_parts(data as *const Self, len)
    }

    fn as_byte_slice(slice: &[Self]) -> &[u8] {
        let data = slice.as_ptr();
        let len = slice.len() * 3;
        unsafe { core::slice::from_raw_parts(data as *const u8, len) }
    }
}

impl AsULE for GenericPatternItem {
    type ULE = EncodedGenericPatternItem;

    #[inline]
    fn as_unaligned(&self) -> Self::ULE {
        match self {
            Self::Placeholder(idx) => EncodedGenericPatternItem([0x00, 0x00, *idx]),
            Self::Literal(ch) => {
                let u = *ch as u32;
                let bytes = u.to_be_bytes();
                EncodedGenericPatternItem([
                    EncodedGenericPatternItem::set_literal_in_u8(bytes[1]),
                    bytes[2],
                    bytes[3],
                ])
            }
        }
    }

    #[inline]
    fn from_unaligned(unaligned: &Self::ULE) -> Self {
        let value = unaligned.0;
        match EncodedGenericPatternItem::is_literal_from_u8(value[0]) {
            false => GenericPatternItem::Placeholder(value[2]),
            true => {
                let first_cleared = value[0] ^ 0b1000_0000;
                let u = u32::from_be_bytes([0x00, first_cleared, value[1], value[2]]);
                GenericPatternItem::Literal(char::try_from(u).unwrap())
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum GenericPatternItem {
    Placeholder(u8),
    Literal(char),
}

impl From<u8> for GenericPatternItem {
    fn from(input: u8) -> Self {
        Self::Placeholder(input)
    }
}

impl<'p> From<char> for GenericPatternItem {
    fn from(input: char) -> Self {
        Self::Literal(input)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct GenericPattern<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub items: ZeroVec<'data, GenericPatternItem>,
}

impl GenericPattern<'_> {
    pub fn from_bytes(input: &str) -> Result<Self, Error> {
        Parser::new(input).parse_generic().map(Self::from)
    }

    pub fn combined<'a>(self, date: Pattern, time: Pattern) -> Result<Pattern<'a>, Error> {
        let mut result = Vec::with_capacity(self.items.len() + date.items.len() + time.items.len());

        for item in self.items.iter() {
            match item {
                GenericPatternItem::Placeholder(idx) => match idx {
                    0 => result.extend(date.items.iter()),
                    1 => result.extend(time.items.iter()),
                    _ => panic!(),
                },
                GenericPatternItem::Literal(ch) => result.push(PatternItem::Literal(ch)),
            }
        }

        Ok(result.into())
    }
}

impl From<Vec<GenericPatternItem>> for GenericPattern<'_> {
    fn from(items: Vec<GenericPatternItem>) -> Self {
        Self {
            items: ZeroVec::clone_from_slice(&items),
        }
    }
}

impl<'data> From<ZeroVec<'data, GenericPatternItem>> for GenericPattern<'data> {
    fn from(items: ZeroVec<'data, GenericPatternItem>) -> Self {
        Self { items }
    }
}

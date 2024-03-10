use core::str::FromStr;

use crate::alloc::string::ToString;
use crate::parser::ParserError;
use alloc::string::String;

#[derive(Debug, PartialEq, Eq)]
enum ParserState {
    Consuming,
    Delimiter,
    End,
}

#[derive(Debug)]
pub struct Language2(tinystr::TinyAsciiStr<3>);

impl Language2 {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    #[inline]
    fn try_from_utf8_slice(s: &[u8]) -> Result<(Self, usize, ParserState), ParserError> {
        let (len, state) = match s.get(2) {
            Some(b) if *b == b'-' as u8 => (2, ParserState::Delimiter),
            None => {
                if s.len() < 2 {
                    return Err(ParserError::InvalidLanguage);
                }
                (2, ParserState::End)
            }
            _ => (3, ParserState::Consuming),
        };
        let value = tinystr::TinyAsciiStr::from_bytes_inner2(&s[..len], len).unwrap();
        if !value.is_ascii_alphabetic() {
            return Err(ParserError::InvalidLanguage);
        }
        Ok((Self(value.to_ascii_lowercase()), len, state))
    }

    #[inline]
    fn try_from_utf16_slice(s: &[u16]) -> Result<(Self, usize, ParserState), ParserError> {
        let (len, state) = match s.get(2) {
            Some(b) if *b == b'-' as u16 => (2, ParserState::Delimiter),
            None => {
                if s.len() < 2 {
                    return Err(ParserError::InvalidLanguage);
                }
                (2, ParserState::End)
            }
            _ => (3, ParserState::Consuming),
        };
        let value = tinystr::TinyAsciiStr::from_bytes_inner2_utf16(&s[..len], len).unwrap();
        if !value.is_ascii_alphabetic() {
            return Err(ParserError::InvalidLanguage);
        }
        Ok((Self(value.to_ascii_lowercase()), len, state))
    }

    #[inline]
    pub fn eq_ignoring_case(&self, input: &[u8]) -> bool {
        self.0.as_bytes().eq_ignore_ascii_case(input)
    }
}

impl PartialEq<[u8]> for Language2 {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool {
        self.0.as_bytes() == other
    }
}

impl PartialEq<[u16]> for Language2 {
    #[inline]
    fn eq(&self, other: &[u16]) -> bool {
        todo!()
    }
}

impl TryFrom<&[u8]> for Language2 {
    type Error = ParserError;

    #[inline]
    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        let slen = input.len();

        if slen < 2 || slen > 3 {
            return Err(ParserError::InvalidLanguage);
        }

        match tinystr::TinyAsciiStr::from_bytes_manual_slice(input, 0, slen) {
            Ok(s) if s.is_ascii_alphabetic() => Ok(Self(s.to_ascii_lowercase())),
            _ => Err(ParserError::InvalidLanguage),
        }
        // let (value, len, _) = Self::try_from_utf8_slice(input)?;
        // if len < input.len() {
        //     return Err(ParserError::InvalidLanguage);
        // }
        // Ok(value)
    }
}

impl TryFrom<&[u16]> for Language2 {
    type Error = ParserError;

    #[inline]
    fn try_from(input: &[u16]) -> Result<Self, Self::Error> {
        let (value, len, _) = Self::try_from_utf16_slice(input)?;
        if len < input.len() {
            return Err(ParserError::InvalidLanguage);
        }
        Ok(value)
    }
}

impl FromStr for Language2 {
    type Err = ParserError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.as_bytes())
    }
}

use crate::parser::errors::ParserError;
use alloc::string::{String, ToString};
use tinystr::tinystr;

#[derive(Debug)]
pub struct LanguageIdentifier2 {
    pub language: Language2,
    pub region: Option<Region2>,
}

#[derive(Debug, PartialEq, Eq)]
enum ParserState {
    Consuming,
    Delimiter,
    End,
}

#[derive(Debug)]
struct Scope {
    pub ptr: usize,
    pub state: ParserState,
}

impl LanguageIdentifier2 {
    pub fn to_string(&self) -> String {
        let mut result = self.language.to_string();
        if let Some(region) = &self.region {
            result.push('-');
            result.push_str(&region.to_string());
        }
        result
    }

    #[inline]
    pub fn try_from_utf8(mut s: &[u8]) -> Result<Self, ParserError> {
        let (value, len) = Self::try_from_utf8_slice(s)?;
        if len < s.len() {
            return Err(ParserError::InvalidSubtag);
        }
        Ok(value)
    }

    #[inline]
    pub fn try_from_utf8_slice(mut s: &[u8]) -> Result<(Self, usize), ParserError> {
        let mut total = 0;
        let (language, len, state) = Language2::try_from_utf8_slice(s)?;
        total += len;
        let mut region = None;
        if state == ParserState::Delimiter {
            s = &s[len + 1..];
            let (value, len) = Region2::try_from_utf8_slice(s)?;
            total += len + 1;
            region = Some(value);
        } else if state == ParserState::Consuming {
            s = &s[len..];
            if let Some(ch) = s.first() {
                if *ch != '-' as u8 {
                    return Err(ParserError::InvalidSubtag);
                } else {
                    s = &s[1..];
                }
            }
            let (value, len) = Region2::try_from_utf8_slice(s)?;
            total += len + 1;
            region = Some(value);
        }
        Ok((Self { language, region }, total))
    }

    pub fn try_from_utf16_slice(mut s: &[u16]) -> Result<(Self, usize), ParserError> {
        let mut total = 0;
        let (language, len, state) = Language2::try_from_utf16_slice(s)?;
        total += len;
        let mut region = None;
        if state == ParserState::Delimiter {
            s = &s[len + 1..];
            let (value, len) = Region2::try_from_utf16_slice(s)?;
            total += len + 1;
            region = Some(value);
        } else if state == ParserState::Consuming {
            s = &s[len..];
            if let Some(ch) = s.first() {
                if *ch != '-' as u16 {
                    return Err(ParserError::InvalidSubtag);
                } else {
                    s = &s[1..];
                }
            }
            let (value, len) = Region2::try_from_utf16_slice(s)?;
            total += len + 1;
            region = Some(value);
        }
        Ok((Self { language, region }, total))
    }

    pub fn eq(&self, mut input: &[u8]) -> bool {
        let mut iter = input.split(|&b| b == b'-' as u8);

        let Some(lang) = iter.next() else {
            return false;
        };
        if !self.language.eq(lang) {
            return false;
        }

        match (iter.next(), &self.region) {
            (Some(candidate), Some(region)) => region.eq(candidate),
            (None, None) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn eq_ignoring_case(&self, mut input: &[u8]) -> bool {
        let mut iter = input.split(|&b| b == b'-' as u8);

        let lang = iter
            .next()
            .is_some_and(|lang| self.language.eq_ignoring_case(lang));
        if !lang {
            return false;
        }

        match (iter.next(), &self.region) {
            (Some(candidate), Some(region)) => region.eq_ignoring_case(candidate),
            (None, None) => true,
            _ => false,
        }
    }
}

impl TryFrom<&[u8]> for LanguageIdentifier2 {
    type Error = ParserError;

    #[inline]
    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        let (value, len) = Self::try_from_utf8_slice(input)?;
        if len < input.len() {
            return Err(ParserError::InvalidSubtag);
        }
        Ok(value)
    }
}

impl TryFrom<&[u16]> for LanguageIdentifier2 {
    type Error = ParserError;

    #[inline]
    fn try_from(input: &[u16]) -> Result<Self, Self::Error> {
        let (value, len) = Self::try_from_utf16_slice(input)?;
        if len < input.len() {
            return Err(ParserError::InvalidSubtag);
        }
        Ok(value)
    }
}

#[derive(Debug)]
pub struct Language2 {
    value: tinystr::TinyAsciiStr<3>,
}

impl Language2 {
    pub fn to_string(&self) -> String {
        self.value.to_string()
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
        Ok((
            Self {
                value: value.to_ascii_lowercase(),
            },
            len,
            state,
        ))
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
        Ok((
            Self {
                value: value.to_ascii_lowercase(),
            },
            len,
            state,
        ))
    }

    #[inline]
    pub fn eq_ignoring_case(&self, input: &[u8]) -> bool {
        self.value.as_bytes().eq_ignore_ascii_case(input)
    }
}

impl PartialEq<[u8]> for Language2 {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool {
        self.value.as_bytes() == other
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
        let (value, len, _) = Self::try_from_utf8_slice(input)?;
        if len < input.len() {
            return Err(ParserError::InvalidLanguage);
        }
        Ok(value)
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

#[derive(Debug)]
pub struct Region2 {
    value: tinystr::TinyAsciiStr<3>,
}

impl Region2 {
    pub fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn try_from_utf8_slice(s: &[u8]) -> Result<(Self, usize), ParserError> {
        let len = if let Some(ch) = s.first() {
            if ch.is_ascii_digit() {
                3
            } else {
                2
            }
        } else {
            return Err(ParserError::InvalidSubtag);
        };
        if s.len() < len {
            return Err(ParserError::InvalidSubtag);
        }
        let value = tinystr::TinyAsciiStr::from_bytes_inner2(&s[..len], len).unwrap();
        if len == 2 {
            if !value.is_ascii_alphabetic() {
                return Err(ParserError::InvalidSubtag);
            }
        } else {
            if !value.is_ascii_numeric() {
                return Err(ParserError::InvalidSubtag);
            }
        }
        Ok((
            Self {
                value: value.to_ascii_uppercase(),
            },
            len,
        ))
    }

    fn try_from_utf16_slice(s: &[u16]) -> Result<(Self, usize), ParserError> {
        let len = if let Some(ch) = s.first() {
            if *ch >= b'0' as u16 && *ch <= b'9' as u16 {
                3
            } else {
                2
            }
        } else {
            return Err(ParserError::InvalidSubtag);
        };
        if s.len() < len {
            return Err(ParserError::InvalidSubtag);
        }
        let value = tinystr::TinyAsciiStr::from_bytes_inner2_utf16(&s[..len], len).unwrap();
        if len == 2 {
            if !value.is_ascii_alphabetic() {
                return Err(ParserError::InvalidSubtag);
            }
        } else {
            if !value.is_ascii_numeric() {
                return Err(ParserError::InvalidSubtag);
            }
        }
        Ok((
            Self {
                value: value.to_ascii_uppercase(),
            },
            len,
        ))
    }

    #[inline]
    pub fn eq_ignoring_case(&self, input: &[u8]) -> bool {
        self.value.as_bytes().eq_ignore_ascii_case(input)
    }

    #[inline]
    pub fn eq(&self, input: &[u8]) -> bool {
        self.value.as_bytes() == input
    }
}

impl TryFrom<&[u8]> for Region2 {
    type Error = ParserError;

    #[inline]
    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        let (value, len) = Self::try_from_utf8_slice(input)?;
        if len < input.len() {
            return Err(ParserError::InvalidSubtag);
        }
        Ok(value)
    }
}

impl TryFrom<&[u16]> for Region2 {
    type Error = ParserError;

    #[inline]
    fn try_from(input: &[u16]) -> Result<Self, Self::Error> {
        let (value, len) = Self::try_from_utf16_slice(input)?;
        if len < input.len() {
            return Err(ParserError::InvalidSubtag);
        }
        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::subtags::*;
    use crate::LanguageIdentifier;

    #[test]
    fn test_locid2_language() {
        let valid = &["pl", "und", "es", "fr", "PL", "pL"];
        let invalid = &["", "0", "abcdefghi", "abcd", "f-f", "-", "a-", "--", "_"];

        for s in valid {
            let canonical_str = Language::try_from_bytes(s.as_bytes()).unwrap().to_string();

            let language = Language2::try_from(s.as_bytes());
            assert!(language.is_ok(), "Expected: {}, got error", s);
            let language = language.unwrap();
            assert_eq!(language.to_string().as_str(), canonical_str,);

            for s2 in valid {
                assert_eq!(
                    s.to_ascii_lowercase() == s2.to_ascii_lowercase(),
                    language.eq_ignoring_case(s2.as_bytes()),
                    "{} == {}",
                    s,
                    s2
                );
                assert_eq!(&canonical_str == *s2, language.eq(s2.as_bytes()));
            }

            for s2 in invalid {
                assert!(!language.eq_ignoring_case(s2.as_bytes()), "{} != {}", s, s2);
                assert!(!language.eq(s2.as_bytes()));
            }

            let s_utf16: Vec<u16> = s.encode_utf16().collect();

            let language_utf16 = Language2::try_from(s_utf16.as_slice());
            assert!(language_utf16.is_ok(), "Expected: {}, got error", s);
            let language_utf16 = language_utf16.unwrap();
            assert_eq!(language_utf16.to_string().as_str(), canonical_str,);

            for s2 in valid {
                assert_eq!(
                    s.to_ascii_lowercase() == s2.to_ascii_lowercase(),
                    language_utf16.eq_ignoring_case(s2.as_bytes())
                );
                assert_eq!(&canonical_str == *s2, language_utf16.eq(s2.as_bytes()));
            }
        }

        for s in invalid {
            let language = Language2::try_from(s.as_bytes());
            assert!(language.is_err(), "Input: {:?}, got {:?}", s, language);

            let s_utf16: Vec<u16> = s.encode_utf16().collect();

            let language_utf16 = Language2::try_from(s_utf16.as_slice());
            assert!(language_utf16.is_err(), "{:?}", language);
        }
    }

    #[test]
    fn test_locid2_region() {
        let valid = &["us", "de", "419", "PL", "pL"];
        let invalid = &["", "0", "abc", "f-", "-", "-f", "--", "_"];

        for s in valid {
            let canonical_str = Region::try_from_bytes(s.as_bytes()).unwrap().to_string();

            let region = Region2::try_from(s.as_bytes());
            assert!(region.is_ok(), "{}", s);
            let region = region.unwrap();
            assert_eq!(region.to_string().as_str(), &canonical_str);

            for s2 in valid {
                assert_eq!(
                    s.to_ascii_lowercase() == s2.to_ascii_lowercase(),
                    region.eq_ignoring_case(s2.as_bytes()),
                    "{} == {}",
                    s,
                    s2
                );
                assert_eq!(&canonical_str == *s2, region.eq(s2.as_bytes()));
            }

            for s2 in invalid {
                assert!(!region.eq_ignoring_case(s2.as_bytes()), "{} != {}", s, s2);
                assert!(!region.eq(s2.as_bytes()));
            }

            let s_utf16: Vec<u16> = s.encode_utf16().collect();

            let region_utf16 = Region2::try_from(s_utf16.as_slice());
            assert!(region_utf16.is_ok());
            assert_eq!(
                region_utf16.unwrap().to_string().as_str(),
                s.to_ascii_uppercase()
            );
        }

        for s in invalid {
            let region = Region2::try_from(s.as_bytes());
            assert!(region.is_err(), "{:?}", s);

            let s_utf16: Vec<u16> = s.encode_utf16().collect();

            let region_utf16 = Region2::try_from(s_utf16.as_slice());
            assert!(region_utf16.is_err(), "{:?}", s);
        }
    }

    #[test]
    fn test_locid2_langid() {
        let valid = &["pl-PL", "und-DE", "es-ES", "fr-FR"];
        let invalid = &[
            "",
            "0",
            "abcdefghi",
            "abcdefghi-419",
            "abcd",
            "f-f",
            "-",
            "a-",
            "--",
            "_",
            "abcde-FR",
            "abcdef-DE",
            "abcdefg-419",
            "abcdefgh-419",
        ];

        for s in valid {
            let canonical_str = LanguageIdentifier::try_from_bytes(s.as_bytes())
                .unwrap()
                .to_string();

            let lid = LanguageIdentifier2::try_from_utf8(s.as_bytes());
            assert!(lid.is_ok(), "{:#?}", s);
            let lid = lid.unwrap();
            assert_eq!(lid.to_string().as_str(), *s);

            for s2 in valid {
                assert_eq!(
                    canonical_str == LanguageIdentifier::canonicalize(s2).unwrap(),
                    lid.eq_ignoring_case(s2.as_bytes()),
                    "{} == {}",
                    s,
                    s2
                );
                assert_eq!(&canonical_str == *s2, lid.eq(s2.as_bytes()));
            }

            for s2 in invalid {
                assert!(!lid.eq_ignoring_case(s2.as_bytes()), "{} != {}", s, s2);
                assert!(!lid.eq(s2.as_bytes()));
            }

            let s_utf16: Vec<u16> = s.encode_utf16().collect();

            let lid_utf16 = LanguageIdentifier2::try_from(s_utf16.as_slice());
            assert!(lid_utf16.is_ok(), "{:#?}", s);
            assert_eq!(lid_utf16.unwrap().to_string().as_str(), *s);
        }

        for s in invalid {
            let lid = LanguageIdentifier2::try_from(s.as_bytes());
            assert!(lid.is_err(), "{:?}", lid);

            let s_utf16: Vec<u16> = s.encode_utf16().collect();

            let lid_utf16 = LanguageIdentifier2::try_from(s_utf16.as_slice());
            assert!(lid_utf16.is_err(), "{:?}", lid);
        }
    }
}

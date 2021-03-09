// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
use super::error::Error;
use super::{Pattern, PatternItem};
use crate::fields::FieldSymbol;
use std::borrow::Cow;
use std::convert::{TryFrom, TryInto};
use std::ops::Range;

#[derive(Debug, PartialEq)]
enum ParserState {
    Literal,
    QuotedLiteral,
    Apostrophe { quoted: bool },
    Token { symbol: FieldSymbol, literal: u8 },
}

pub trait Slice<'p> {
    fn get(&self, idx: usize) -> Option<u8>;
    fn get_slice(&self, range: Range<usize>) -> Cow<'p, str>;
    fn length(&self) -> usize;
}

impl<'p> Slice<'p> for Cow<'p, str> {
    fn get(&self, idx: usize) -> Option<u8> {
        self.as_bytes().get(idx).copied()
    }

    fn get_slice(&self, range: Range<usize>) -> Cow<'p, str> {
        match self {
            Self::Borrowed(b) => Cow::Borrowed(&b[range]),
            Self::Owned(o) => Cow::Owned(o[range].to_string()),
        }
    }

    fn length(&self) -> usize {
        self.len()
    }
}

impl<'p> Slice<'p> for &'p str {
    fn get(&self, idx: usize) -> Option<u8> {
        self.as_bytes().get(idx).copied()
    }

    fn get_slice(&self, range: Range<usize>) -> Cow<'p, str> {
        Cow::Borrowed(&self[range])
    }

    fn length(&self) -> usize {
        self.len()
    }
}

pub struct Parser<'p, 's, S> {
    input: &'s S,
    start_idx: usize,
    idx: usize,
    len: usize,
    state: ParserState,
    marker: std::marker::PhantomData<&'p S>,
}

impl<'p, 's, S> Parser<'p, 's, S> {
    pub fn new(input: &'s S) -> Self
    where
        S: Slice<'p>,
    {
        let len = input.length();
        Self {
            input,
            start_idx: 0,
            idx: 0,
            len,
            state: ParserState::Literal,
            marker: std::marker::PhantomData,
        }
    }
}

impl<'p, 's, S> Iterator for Parser<'p, 's, S>
where
    S: Slice<'p>,
{
    type Item = Result<PatternItem<'p>, Error>;

    fn next(&mut self) -> std::option::Option<<Self as Iterator>::Item> {
        #[cfg(debug_assertions)]
        println!(
            "next: {:?}, idx: {}, start_idx: {}",
            self.state, self.idx, self.start_idx
        );
        if self.start_idx == self.len {
            return None;
        }
        while let Some(b) = self.input.get(self.idx) {
            #[cfg(debug_assertions)]
            println!(
                "while: {:?}, idx: {}, start_idx: {}, b: {}",
                self.state, self.idx, self.start_idx, b
            );
            match self.state {
                ParserState::Apostrophe { quoted } => {
                    self.state = if quoted {
                        ParserState::QuotedLiteral
                    } else {
                        ParserState::Literal
                    };
                    self.idx += 1;
                    let range = self.start_idx..self.idx;
                    self.start_idx = self.idx;
                    return Some(Ok(PatternItem::Literal(self.input.get_slice(range))));
                }
                ParserState::QuotedLiteral => {
                    if b == b'\'' {
                        let range = self.start_idx..self.idx;

                        self.idx += 1;
                        self.start_idx = self.idx;

                        self.state = if let Some(b'\'') = self.input.get(self.idx) {
                            ParserState::Apostrophe { quoted: true }
                        } else {
                            ParserState::Literal
                        };

                        if !range.is_empty() {
                            return Some(Ok(PatternItem::Literal(self.input.get_slice(range))));
                        }
                    } else {
                        self.idx += 1;
                    }
                }
                ParserState::Literal => {
                    if b.is_ascii_alphabetic() {
                        if let Ok(symbol) = FieldSymbol::try_from(b) {
                            let range = self.start_idx..self.idx;
                            self.state = ParserState::Token { symbol, literal: b };
                            self.start_idx = self.idx;
                            self.idx += 1;
                            if !range.is_empty() {
                                return Some(Ok(PatternItem::Literal(self.input.get_slice(range))));
                            }
                        } else {
                            return Some(Err(Error::UnknownField(b as char)));
                        }
                    } else if b == b'\'' {
                        let range = self.start_idx..self.idx;

                        self.idx += 1;
                        self.start_idx = self.idx;

                        self.state = match self.input.get(self.idx) {
                            Some(b'\'') => ParserState::Apostrophe { quoted: false },
                            Some(_) => ParserState::QuotedLiteral,
                            None => return Some(Err(Error::UnclosedLiteral)),
                        };

                        if !range.is_empty() {
                            return Some(Ok(PatternItem::Literal(self.input.get_slice(range))));
                        }
                    } else {
                        self.idx += 1;
                    }
                }
                ParserState::Token { symbol, literal } if literal != b => {
                    let length = self.idx - self.start_idx;
                    self.start_idx = self.idx;
                    self.state = ParserState::Literal;
                    if let Ok(field) = (symbol, length).try_into() {
                        return Some(Ok(PatternItem::Field(field)));
                    } else {
                        return Some(Err(Error::FieldTooLong(symbol)));
                    }
                }
                _ => {
                    self.idx += 1;
                }
            }
        }
        #[cfg(debug_assertions)]
        println!("end: {:?}, start_idx: {}", self.state, self.start_idx);
        let range = self.start_idx..self.len;
        self.start_idx = self.len;
        match self.state {
            ParserState::Literal if range.is_empty() => None,
            ParserState::Literal => Some(Ok(PatternItem::Literal(self.input.get_slice(range)))),
            ParserState::Token { symbol, .. } => {
                let length = range.end - range.start;
                if let Ok(field) = (symbol, length).try_into() {
                    Some(Ok(PatternItem::Field(field)))
                } else {
                    Some(Err(Error::FieldTooLong(symbol)))
                }
            }
            ParserState::QuotedLiteral => Some(Err(Error::UnclosedLiteral)),
            ParserState::Apostrophe { .. } => {
                unreachable!();
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::fields::{self, FieldLength};
//     use crate::pattern::PatternItem;

//     #[test]
//     fn pattern_parse_perf() {
//         let samples = vec!["' '"];

//         for string in samples {
//             parse(&string.into())
//                 .collect::<Result<Vec<PatternItem>, _>>()
//                 .unwrap();
//         }
//     }

//     #[test]
//     fn pattern_parse_simple() {
//         let samples = vec![
//             (
//                 "dd/MM/y",
//                 vec![
//                     (fields::Day::DayOfMonth.into(), FieldLength::TwoDigit).into(),
//                     "/".into(),
//                     (fields::Month::Format.into(), FieldLength::TwoDigit).into(),
//                     "/".into(),
//                     (fields::Year::Calendar.into(), FieldLength::One).into(),
//                 ],
//             ),
//             (
//                 "HH:mm:ss",
//                 vec![
//                     (fields::Hour::H23.into(), FieldLength::TwoDigit).into(),
//                     ":".into(),
//                     (FieldSymbol::Minute, FieldLength::TwoDigit).into(),
//                     ":".into(),
//                     (fields::Second::Second.into(), FieldLength::TwoDigit).into(),
//                 ],
//             ),
//             (
//                 "y年M月d日",
//                 vec![
//                     (fields::Year::Calendar.into(), FieldLength::One).into(),
//                     "年".into(),
//                     (fields::Month::Format.into(), FieldLength::One).into(),
//                     "月".into(),
//                     (fields::Day::DayOfMonth.into(), FieldLength::One).into(),
//                     "日".into(),
//                 ],
//             ),
//         ];

//         for (string, pattern_items) in samples {
//             assert_eq!(
//                 parse(&string.into())
//                     .collect::<Result<Vec<PatternItem>, _>>()
//                     .unwrap(),
//                 pattern_items
//             );
//         }
//     }

//     #[test]
//     fn pattern_parse_placeholder_simple() {
//         let samples = vec![(
//             "{1} 'at' {0}",
//             vec![
//                 vec![(fields::Day::DayOfMonth.into(), FieldLength::TwoDigit).into()],
//                 vec![(fields::Year::Calendar.into(), FieldLength::One).into()],
//             ],
//             vec![
//                 (fields::Year::Calendar.into(), FieldLength::One).into(),
//                 " ".into(),
//                 "at".into(),
//                 " ".into(),
//                 (fields::Day::DayOfMonth.into(), FieldLength::TwoDigit).into(),
//             ],
//         )];

//         for (string, replacements, pattern_items) in samples {
//             let replacements = replacements.into_iter().map(Into::into).collect();
//             assert_eq!(
//                 PlaceholderParser::new(&string, replacements)
//                     .collect::<Result<Vec<PatternItem>, _>>()
//                     .unwrap(),
//                 pattern_items
//             );
//         }
//     }

//     #[test]
//     fn pattern_parse_literals() {
//         let samples = vec![
//             ("", vec![]),
//             (" ", vec![" ".into()]),
//             ("  ", vec!["  ".into()]),
//             (" żółć ", vec![" żółć ".into()]),
//             ("''", vec!["'".into()]),
//             (" ''", vec![" ".into(), "'".into()]),
//             (" '' ", vec![" ".into(), "'".into(), " ".into()]),
//             ("''''", vec!["'".into(), "'".into()]),
//             (
//                 " '' '' ",
//                 vec![" ".into(), "'".into(), " ".into(), "'".into(), " ".into()],
//             ),
//             ("ż'ół'ć", vec!["ż".into(), "ół".into(), "ć".into()]),
//             (
//                 "ż'ó''ł'ć",
//                 vec!["ż".into(), "ó".into(), "'".into(), "ł".into(), "ć".into()],
//             ),
//             (" 'Ymd' ", vec![" ".into(), "Ymd".into(), " ".into()]),
//             ("الأسبوع", vec!["الأسبوع".into()]),
//         ];

//         for (string, pattern_items) in samples {
//             assert_eq!(
//                 Parser::new(&string)
//                     .collect::<Result<Vec<PatternItem>, _>>()
//                     .unwrap(),
//                 pattern_items
//             );

//             assert_eq!(
//                 PlaceholderParser::new(&string, vec![])
//                     .collect::<Result<Vec<PatternItem>, _>>()
//                     .unwrap(),
//                 pattern_items,
//             );
//         }

//         let broken = vec![
//             (" 'foo ", Error::UnclosedLiteral),
//             (" '", Error::UnclosedLiteral),
//         ];

//         for (string, error) in broken {
//             assert!(Parser::new(&string)
//                 .collect::<Result<Vec<PatternItem>, _>>()
//                 .is_err(),);
//         }
//     }

//     #[test]
//     fn pattern_parse_symbols() {
//         let samples = vec![
//             (
//                 "y",
//                 vec![(fields::Year::Calendar.into(), FieldLength::One).into()],
//             ),
//             (
//                 "yy",
//                 vec![(fields::Year::Calendar.into(), FieldLength::TwoDigit).into()],
//             ),
//             (
//                 "yyy",
//                 vec![(fields::Year::Calendar.into(), FieldLength::Abbreviated).into()],
//             ),
//             (
//                 "yyyy",
//                 vec![(fields::Year::Calendar.into(), FieldLength::Wide).into()],
//             ),
//             (
//                 "yyyyy",
//                 vec![(fields::Year::Calendar.into(), FieldLength::Narrow).into()],
//             ),
//             (
//                 "yyyyyy",
//                 vec![(fields::Year::Calendar.into(), FieldLength::Six).into()],
//             ),
//             (
//                 "yM",
//                 vec![
//                     (fields::Year::Calendar.into(), FieldLength::One).into(),
//                     (fields::Month::Format.into(), FieldLength::One).into(),
//                 ],
//             ),
//             (
//                 "y ",
//                 vec![
//                     (fields::Year::Calendar.into(), FieldLength::One).into(),
//                     " ".into(),
//                 ],
//             ),
//             (
//                 "y M",
//                 vec![
//                     (fields::Year::Calendar.into(), FieldLength::One).into(),
//                     " ".into(),
//                     (fields::Month::Format.into(), FieldLength::One).into(),
//                 ],
//             ),
//             (
//                 "hh''a",
//                 vec![
//                     (fields::Hour::H12.into(), FieldLength::TwoDigit).into(),
//                     "'".into(),
//                     (fields::DayPeriod::AmPm.into(), FieldLength::One).into(),
//                 ],
//             ),
//             (
//                 "hh''b",
//                 vec![
//                     (fields::Hour::H12.into(), FieldLength::TwoDigit).into(),
//                     "'".into(),
//                     (fields::DayPeriod::NoonMidnight.into(), FieldLength::One).into(),
//                 ],
//             ),
//             (
//                 "y'My'M",
//                 vec![
//                     (fields::Year::Calendar.into(), FieldLength::One).into(),
//                     "My".into(),
//                     (fields::Month::Format.into(), FieldLength::One).into(),
//                 ],
//             ),
//             (
//                 "y 'My' M",
//                 vec![
//                     (fields::Year::Calendar.into(), FieldLength::One).into(),
//                     " ".into(),
//                     "My".into(),
//                     " ".into(),
//                     (fields::Month::Format.into(), FieldLength::One).into(),
//                 ],
//             ),
//             (
//                 " 'r'. 'y'. ",
//                 vec![" ".into(), "r".into(), ". ".into(), "y".into(), ". ".into()],
//             ),
//             (
//                 "hh 'o''clock' a",
//                 vec![
//                     (fields::Hour::H12.into(), FieldLength::TwoDigit).into(),
//                     " ".into(),
//                     "o".into(),
//                     "'".into(),
//                     "clock".into(),
//                     " ".into(),
//                     (fields::DayPeriod::AmPm.into(), FieldLength::One).into(),
//                 ],
//             ),
//             (
//                 "hh 'o''clock' b",
//                 vec![
//                     (fields::Hour::H12.into(), FieldLength::TwoDigit).into(),
//                     " ".into(),
//                     "o".into(),
//                     "'".into(),
//                     "clock".into(),
//                     " ".into(),
//                     (fields::DayPeriod::NoonMidnight.into(), FieldLength::One).into(),
//                 ],
//             ),
//             (
//                 "hh''a",
//                 vec![
//                     (fields::Hour::H12.into(), FieldLength::TwoDigit).into(),
//                     "'".into(),
//                     (fields::DayPeriod::AmPm.into(), FieldLength::One).into(),
//                 ],
//             ),
//             (
//                 "hh''b",
//                 vec![
//                     (fields::Hour::H12.into(), FieldLength::TwoDigit).into(),
//                     "'".into(),
//                     (fields::DayPeriod::NoonMidnight.into(), FieldLength::One).into(),
//                 ],
//             ),
//         ];

//         for (string, pattern_items) in samples {
//             assert_eq!(
//                 Parser::new(&string)
//                     .collect::<Result<Vec<PatternItem>, _>>()
//                     .unwrap(),
//                 pattern_items
//             );
//         }

//         let broken = vec![(
//             "yyyyyyy",
//             Error::FieldTooLong(FieldSymbol::Year(fields::Year::Calendar)),
//         )];

//         for (string, error) in broken {
//             assert!(Parser::new(&string.into())
//                 .collect::<Result<Vec<PatternItem>, _>>()
//                 .is_err(),);
//         }
//     }
// }

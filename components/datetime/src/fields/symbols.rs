// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields::FieldLength;
use core::{cmp::Ordering, convert::TryFrom};
use displaydoc::Display;
use zerovec::ule::{AsULE, ULE};

#[derive(Display, Debug, PartialEq)]
pub enum SymbolError {
    /// Unknown field symbol.
    #[displaydoc("Unknown field symbol: {0}")]
    Unknown(u8),
    /// Invalid character for a field symbol.
    #[displaydoc("Invalid character for a field symbol: {0}")]
    Invalid(char),
}

#[cfg(feature = "std")]
impl std::error::Error for SymbolError {}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum FieldSymbol {
    Year(Year),
    Month(Month),
    Day(Day),
    Weekday(Weekday),
    DayPeriod(DayPeriod),
    Hour(Hour),
    Minute,
    Second(Second),
    TimeZone(TimeZone),
}

impl From<u8> for FieldSymbol {
    fn from(input: u8) -> Self {
        let lower = input & 0b0000_1111;
        let upper = input >> 4;

        match lower {
            0 => Self::Year(Year::from(upper)),
            1 => Self::Month(Month::from(upper)),
            2 => Self::Day(Day::from(upper)),
            3 => Self::Weekday(Weekday::from(upper)),
            4 => Self::DayPeriod(DayPeriod::from(upper)),
            5 => Self::Hour(Hour::from(upper)),
            6 => Self::Minute,
            7 => Self::Second(Second::from(upper)),
            8 => Self::TimeZone(TimeZone::from(upper)),
            _ => panic!(),
        }
    }
}

impl From<FieldSymbol> for u8 {
    fn from(input: FieldSymbol) -> Self {
        let (lower, upper) = match input {
            FieldSymbol::Year(year) => (0b0000, year as u8),
            FieldSymbol::Month(month) => (0b0001, month as u8),
            FieldSymbol::Day(day) => (0b0010, day as u8),
            FieldSymbol::Weekday(wd) => (0b0011, wd as u8),
            FieldSymbol::DayPeriod(dp) => (0b0100, dp as u8),
            FieldSymbol::Hour(hour) => (0b0101, hour as u8),
            FieldSymbol::Minute => (0b0110, 0),
            FieldSymbol::Second(second) => (0b0111, second as u8),
            FieldSymbol::TimeZone(tz) => (0b1000, tz as u8),
        };
        let result = upper << 4;
        result | lower
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum TextOrNumeric {
    Text,
    Numeric,
}

/// [`FieldSymbols`](FieldSymbol) can be either text or numeric. This categorization is important
/// when matching skeletons with a components [`Bag`](crate::options::components::Bag).
pub trait LengthType {
    fn get_length_type(&self, length: FieldLength) -> TextOrNumeric;
}

impl FieldSymbol {
    /// Skeletons are a Vec<Field>, and represent the Fields that can be used to match to a
    /// specific pattern. The order of the Vec does not affect the Pattern that is output.
    /// However, it's more performant when matching these fields, and it's more deterministic
    /// when serializing them to present them in a consistent order.
    ///
    /// This ordering is taken by the order of the fields listed in the [UTS 35 Date Field Symbol Table]
    /// (https://unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table), and are generally
    /// ordered most significant to least significant.
    fn get_canonical_order(&self) -> u8 {
        match self {
            Self::Year(Year::Calendar) => 0,
            Self::Year(Year::WeekOf) => 1,
            Self::Month(Month::Format) => 2,
            Self::Month(Month::StandAlone) => 3,
            Self::Day(Day::DayOfMonth) => 4,
            Self::Day(Day::DayOfYear) => 5,
            Self::Day(Day::DayOfWeekInMonth) => 6,
            Self::Day(Day::ModifiedJulianDay) => 7,
            Self::Weekday(Weekday::Format) => 8,
            Self::Weekday(Weekday::Local) => 9,
            Self::Weekday(Weekday::StandAlone) => 10,
            Self::DayPeriod(DayPeriod::AmPm) => 11,
            Self::DayPeriod(DayPeriod::NoonMidnight) => 12,
            Self::Hour(Hour::H11) => 13,
            Self::Hour(Hour::H12) => 14,
            Self::Hour(Hour::H23) => 15,
            Self::Hour(Hour::H24) => 16,
            Self::Minute => 17,
            Self::Second(Second::Second) => 18,
            Self::Second(Second::FractionalSecond) => 19,
            Self::Second(Second::Millisecond) => 20,
            Self::TimeZone(TimeZone::LowerZ) => 21,
            Self::TimeZone(TimeZone::UpperZ) => 22,
            Self::TimeZone(TimeZone::UpperO) => 23,
            Self::TimeZone(TimeZone::LowerV) => 24,
            Self::TimeZone(TimeZone::UpperV) => 25,
            Self::TimeZone(TimeZone::LowerX) => 26,
            Self::TimeZone(TimeZone::UpperX) => 27,
        }
    }

    pub fn kv_in_range(kv: &u8) -> bool {
        let k = kv & 0b0000_1111;
        let v = kv >> 4;
        match k {
            0 => Year::u8_in_range(&v),
            1 => Month::u8_in_range(&v),
            2 => Day::u8_in_range(&v),
            3 => Weekday::u8_in_range(&v),
            4 => DayPeriod::u8_in_range(&v),
            5 => Hour::u8_in_range(&v),
            6 => true,
            7 => Second::u8_in_range(&v),
            8 => TimeZone::u8_in_range(&v),
            _ => false,
        }
    }
}

impl TryFrom<char> for FieldSymbol {
    type Error = SymbolError;
    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            'm' => Ok(Self::Minute),
            _ => Year::try_from(ch)
                .map(Self::Year)
                .or_else(|_| Month::try_from(ch).map(Self::Month))
                .or_else(|_| Day::try_from(ch).map(Self::Day))
                .or_else(|_| Weekday::try_from(ch).map(Self::Weekday))
                .or_else(|_| DayPeriod::try_from(ch).map(Self::DayPeriod))
                .or_else(|_| Hour::try_from(ch).map(Self::Hour))
                .or_else(|_| Second::try_from(ch).map(Self::Second))
                .or_else(|_| TimeZone::try_from(ch).map(Self::TimeZone)),
        }
    }
}

impl From<FieldSymbol> for char {
    fn from(symbol: FieldSymbol) -> Self {
        match symbol {
            FieldSymbol::Year(year) => match year {
                Year::Calendar => 'y',
                Year::WeekOf => 'Y',
            },
            FieldSymbol::Month(month) => match month {
                Month::Format => 'M',
                Month::StandAlone => 'L',
            },
            FieldSymbol::Day(day) => match day {
                Day::DayOfMonth => 'd',
                Day::DayOfYear => 'D',
                Day::DayOfWeekInMonth => 'F',
                Day::ModifiedJulianDay => 'g',
            },
            FieldSymbol::Weekday(weekday) => match weekday {
                Weekday::Format => 'E',
                Weekday::Local => 'e',
                Weekday::StandAlone => 'c',
            },
            FieldSymbol::DayPeriod(dayperiod) => match dayperiod {
                DayPeriod::AmPm => 'a',
                DayPeriod::NoonMidnight => 'b',
            },
            FieldSymbol::Hour(hour) => match hour {
                Hour::H11 => 'K',
                Hour::H12 => 'h',
                Hour::H23 => 'H',
                Hour::H24 => 'k',
            },
            FieldSymbol::Minute => 'm',
            FieldSymbol::Second(second) => match second {
                Second::Second => 's',
                Second::FractionalSecond => 'S',
                Second::Millisecond => 'A',
            },
            FieldSymbol::TimeZone(time_zone) => match time_zone {
                TimeZone::LowerZ => 'z',
                TimeZone::UpperZ => 'Z',
                TimeZone::UpperO => 'O',
                TimeZone::LowerV => 'v',
                TimeZone::UpperV => 'V',
                TimeZone::LowerX => 'x',
                TimeZone::UpperX => 'X',
            },
        }
    }
}

impl PartialOrd for FieldSymbol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FieldSymbol {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_canonical_order().cmp(&other.get_canonical_order())
    }
}

macro_rules! const_expr_count {
    () => (0);
    ($e:expr) => (1);
    ($e:expr; $($other_e:expr);*) => ({
        1 $(+ const_expr_count!($other_e) )*
    });

    ($e:expr; $($other_e:expr);* ; ) => (
        const_expr_count! { $e; $($other_e);* }
    );
}

macro_rules! field_type {
    ($i:ident; { $($idx:expr; $key:expr => $val:ident),* }) => (
        #[derive(Debug, Eq, PartialEq, Clone, Copy)]
        #[cfg_attr(
            feature = "provider_serde",
            derive(serde::Serialize, serde::Deserialize)
        )]
        #[allow(clippy::enum_variant_names)]
        #[repr(u8)]
        pub enum $i {
            $($val, )*
        }


        impl $i {
            pub fn u8_in_range(v: &u8) -> bool {
                let count = const_expr_count!($($key);*);
                (0..count).contains(v)
            }
        }

        impl From<u8> for $i {
            fn from(input: u8) -> Self {
                match input {
                    $(
                        $idx => Self::$val,
                    )*
                    _ => panic!(),
                }
            }
        }


        impl TryFrom<char> for $i {
            type Error = SymbolError;
            fn try_from(b: char) -> Result<Self, Self::Error> {
                match b {
                    $(
                        $key => Ok(Self::$val),
                    )*
                    //XXX: Fix this - symbol is char
                    b => Err(SymbolError::Unknown(b as u8)),
                }
            }
        }

        impl From<$i> for FieldSymbol {
            fn from(input: $i) -> Self {
                Self::$i(input)
            }
        }


        unsafe impl ULE for $i {
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

        impl AsULE for $i {
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
    );
}

field_type!(Year; {
    0; 'y' => Calendar,
    1; 'Y' => WeekOf
});

field_type!(Month; {
    0; 'M' => Format,
    1; 'L' => StandAlone
});

field_type!(Day; {
    0; 'd' => DayOfMonth,
    1; 'D' => DayOfYear,
    2; 'F' => DayOfWeekInMonth,
    3; 'g' => ModifiedJulianDay
});

field_type!(Weekday; {
    0; 'E' => Format,
    1; 'e' => Local,
    2; 'c' => StandAlone
});

field_type!(DayPeriod; {
    0; 'a' => AmPm,
    1; 'b' => NoonMidnight
});

field_type!(Hour; {
    0; 'K' => H11,
    1; 'h' => H12,
    2; 'H' => H23,
    3; 'k' => H24
});

field_type!(Second; {
    0; 's' => Second,
    1; 'S' => FractionalSecond,
    2; 'A' => Millisecond
});

field_type!(TimeZone; {
    0; 'z' => LowerZ,
    1; 'Z' => UpperZ,
    2; 'O' => UpperO,
    3; 'v' => LowerV,
    4; 'V' => UpperV,
    5; 'x' => LowerX,
    6; 'X' => UpperX
});

impl LengthType for Year {
    fn get_length_type(&self, _length: FieldLength) -> TextOrNumeric {
        TextOrNumeric::Numeric
    }
}

impl LengthType for Month {
    fn get_length_type(&self, length: FieldLength) -> TextOrNumeric {
        match length {
            FieldLength::One => TextOrNumeric::Numeric,
            FieldLength::TwoDigit => TextOrNumeric::Numeric,
            FieldLength::Abbreviated => TextOrNumeric::Text,
            FieldLength::Wide => TextOrNumeric::Text,
            FieldLength::Narrow => TextOrNumeric::Text,
            FieldLength::Six => TextOrNumeric::Text,
        }
    }
}

impl LengthType for Day {
    fn get_length_type(&self, _length: FieldLength) -> TextOrNumeric {
        TextOrNumeric::Numeric
    }
}

impl LengthType for Hour {
    fn get_length_type(&self, _length: FieldLength) -> TextOrNumeric {
        TextOrNumeric::Numeric
    }
}

impl LengthType for Second {
    fn get_length_type(&self, _length: FieldLength) -> TextOrNumeric {
        TextOrNumeric::Numeric
    }
}

impl LengthType for Weekday {
    fn get_length_type(&self, length: FieldLength) -> TextOrNumeric {
        match self {
            Self::Format => TextOrNumeric::Text,
            Self::Local | Self::StandAlone => match length {
                FieldLength::One | FieldLength::TwoDigit => TextOrNumeric::Text,
                _ => TextOrNumeric::Numeric,
            },
        }
    }
}

impl LengthType for DayPeriod {
    fn get_length_type(&self, _length: FieldLength) -> TextOrNumeric {
        TextOrNumeric::Text
    }
}

impl LengthType for TimeZone {
    fn get_length_type(&self, length: FieldLength) -> TextOrNumeric {
        use TextOrNumeric::*;
        match self {
            // It is reasonable to default to Text on release builds instead of panicking.
            //
            // Erroneous symbols are gracefully handled by returning error Results
            // in the formatting code.
            //
            // The default cases may want to be updated to return errors themselves
            // if the skeleton matching code ever becomes fallible.
            Self::UpperZ => match u8::from(length) {
                1..=3 => Numeric,
                4 => Text,
                5 => Numeric,
                _ => Text,
            },
            Self::UpperO => match u8::from(length) {
                1 => Text,
                4 => Numeric,
                _ => Text,
            },
            Self::LowerX | Self::UpperX => Numeric,
            Self::LowerZ | Self::LowerV | Self::UpperV => Text,
        }
    }
}

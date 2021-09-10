use displaydoc::Display;
use std::convert::{TryFrom, TryInto};
use zerovec::ule::{AsULE, ULE};

#[derive(Display, Debug, PartialEq)]
pub enum SymbolError {
    /// Unknown field symbol.
    #[displaydoc("Unknown field symbol: {0}")]
    Unknown(char),
    /// Invalid character for a field symbol.
    #[displaydoc("Invalid character for a field symbol: {0}")]
    Invalid(char),
}

#[cfg(feature = "std")]
impl std::error::Error for SymbolError {}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[repr(u8)]
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

impl FieldSymbol {
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

    fn try_from(b: char) -> Result<Self, Self::Error> {
        match b {
            'm' => Ok(Self::Minute),
            _ => Year::try_from(b)
                .map(Self::Year)
                .or_else(|_| Month::try_from(b).map(Self::Month))
                .or_else(|_| Day::try_from(b).map(Self::Day))
                .or_else(|_| Weekday::try_from(b).map(Self::Weekday))
                .or_else(|_| DayPeriod::try_from(b).map(Self::DayPeriod))
                .or_else(|_| Hour::try_from(b).map(Self::Hour))
                .or_else(|_| Second::try_from(b).map(Self::Second))
                .or_else(|_| TimeZone::try_from(b).map(Self::TimeZone)),
        }
    }
}

impl ULE for FieldSymbol {
    type Error = ();

    fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error> {
        if !bytes.iter().all(|c| FieldSymbol::kv_in_range(c)) {
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

impl AsULE for FieldSymbol {
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
                    b => Err(SymbolError::Unknown(b)),
                }
            }
        }

        impl ULE for $i {
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
    2; 'c' => StandAlone,
    3; 'g' => ModifiedJulianDay
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
    2; 'A' => Mllisecond
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

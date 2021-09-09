use zerovec::ule::{AsULE, ULE};

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[repr(u8)]
pub enum FieldSymbol {
    Year(Year),
    Month(Month),
}

impl From<u8> for FieldSymbol {
    fn from(input: u8) -> Self {
        let lower = input & 0b0000_1111;
        let upper = input >> 4;

        match lower {
            0 => Self::Year(Year::from(upper)),
            1 => Self::Month(Month::from(upper)),
            _ => panic!(),
        }
    }
}

impl From<FieldSymbol> for u8 {
    fn from(input: FieldSymbol) -> Self {
        let (lower, upper) = match input {
            FieldSymbol::Year(year) => (0b0000, year as u8),
            FieldSymbol::Month(month) => (0b0001, month as u8),
        };
        let result = upper << 4;
        result | lower
    }
}

impl FieldSymbol {
    pub fn kv_in_range(k: &u8, v: &u8) -> bool {
        match k {
            0 => Year::u8_in_range(v),
            1 => Month::u8_in_range(v),
            _ => false,
        }
    }
}

impl ULE for FieldSymbol {
    type Error = ();

    fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error> {
        let mut chunks = bytes.chunks_exact(2);

        if !chunks.all(|c| FieldSymbol::kv_in_range(&c[0], &c[1])) || !chunks.remainder().is_empty()
        {
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

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[repr(u8)]
pub enum Year {
    Calendar,
    WeekOf,
}

impl Year {
    pub fn u8_in_range(v: &u8) -> bool {
        (0..2).contains(v)
    }
}

impl From<u8> for Year {
    fn from(input: u8) -> Self {
        match input {
            0 => Self::Calendar,
            1 => Self::WeekOf,
            _ => panic!(),
        }
    }
}

impl ULE for Year {
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

impl AsULE for Year {
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

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[repr(u8)]
pub enum Month {
    Short,
    Long,
}

impl Month {
    pub fn u8_in_range(v: &u8) -> bool {
        (0..2).contains(v)
    }
}

impl From<u8> for Month {
    fn from(input: u8) -> Self {
        match input {
            0 => Self::Short,
            1 => Self::Long,
            _ => panic!(),
        }
    }
}

impl ULE for Month {
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

impl AsULE for Month {
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

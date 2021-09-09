use zerovec::ule::{AsULE, ULE};

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum FieldSymbol {
    Year(Year),
    Month(Month),
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

#[derive(Copy, Clone, Debug, PartialEq)]
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

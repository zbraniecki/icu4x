use std::borrow::Cow;
use std::hash::Hash;
use std::ops::Range;

pub trait Slice<'s>: Hash + PartialEq {
    fn from_str(input: &'s str) -> Self;
    fn slice(&self, range: Range<usize>) -> Self;
    fn byte_at(&self, ptr: usize) -> Option<&u8>;
    fn as_str(&self) -> &str;
    fn into_cow(self) -> Cow<'s, str>;
}

impl<'s> Slice<'s> for String {
    fn from_str(input: &'s str) -> Self {
        String::from(input)
    }

    fn slice(&self, range: Range<usize>) -> Self {
        self[range].to_string()
    }

    fn byte_at(&self, ptr: usize) -> Option<&u8> {
        self.as_bytes().get(ptr)
    }

    fn as_str(&self) -> &str {
        self.as_str()
    }

    fn into_cow(self) -> Cow<'s, str> {
        Cow::Owned(self)
    }
}

impl<'s> Slice<'s> for &'s str {
    fn from_str(input: &'s str) -> Self {
        input
    }

    #[inline]
    fn slice(&self, range: Range<usize>) -> Self {
        &self[range]
    }

    #[inline]
    fn byte_at(&self, ptr: usize) -> Option<&u8> {
        self.as_bytes().get(ptr)
    }

    fn as_str(&self) -> &str {
        self
    }

    fn into_cow(self) -> Cow<'s, str> {
        Cow::Borrowed(self)
    }
}
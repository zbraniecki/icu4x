use std::borrow::Cow;
use std::ops::Range;

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

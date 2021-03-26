use crate::Slice;
use std::{borrow::Cow, ops::Range};

impl<'s> Slice<'s, &'s str> for Cow<'s, str> {
    fn get_byte(&self, idx: usize) -> Option<u8> {
        self.as_bytes().get(idx).copied()
    }

    fn get_str_slice(&self, range: Range<usize>) -> &str {
        &self[range]
    }

    fn get_cow_slice(&self, range: Range<usize>) -> Cow<'s, str> {
        match self {
            Self::Borrowed(b) => Cow::Borrowed(&b[range]),
            Self::Owned(o) => Cow::Owned(o[range].to_owned()),
        }
    }

    fn get_slice(&'s self, range: Range<usize>) -> &'s str {
        match self {
            Self::Borrowed(b) => &b[range],
            Self::Owned(o) => &o[range],
        }
    }

    fn length(&self) -> usize {
        self.len()
    }
}

impl<'s> Slice<'s, String> for Cow<'s, str> {
    fn get_byte(&self, idx: usize) -> Option<u8> {
        self.as_bytes().get(idx).copied()
    }

    fn get_str_slice(&self, range: Range<usize>) -> &str {
        &self[range]
    }

    fn get_cow_slice(&self, range: Range<usize>) -> Cow<'s, str> {
        match self {
            Self::Borrowed(b) => Cow::Borrowed(&b[range]),
            Self::Owned(o) => Cow::Owned(o[range].to_owned()),
        }
    }

    fn get_slice(&'s self, range: Range<usize>) -> String {
        self[range].to_string()
    }

    fn length(&self) -> usize {
        self.len()
    }
}

impl<'s> Slice<'s, Cow<'s, str>> for Cow<'s, str> {
    fn get_byte(&self, idx: usize) -> Option<u8> {
        self.as_bytes().get(idx).copied()
    }

    fn get_str_slice(&self, range: Range<usize>) -> &str {
        &self[range]
    }

    fn get_cow_slice(&self, range: Range<usize>) -> Cow<'s, str> {
        match self {
            Self::Borrowed(b) => Cow::Borrowed(&b[range]),
            Self::Owned(o) => Cow::Owned(o[range].to_owned()),
        }
    }

    fn get_slice(&'s self, range: Range<usize>) -> Cow<'s, str> {
        match self {
            Self::Borrowed(b) => Cow::Borrowed(&b[range]),
            Self::Owned(o) => Cow::Owned(o[range].to_owned()),
        }
    }


    fn length(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct PatternItem<S>(S);

    fn is_cow_owned<T>(input: &Cow<T>) -> bool
    where
        T: ToOwned + ?Sized,
    {
        match input {
            Cow::Borrowed(_) => false,
            Cow::Owned(_) => true,
        }
    }

    #[test]
    fn cow_borrowed_from_cow_borrowed() {
        let pi = PatternItem(Cow::Borrowed("Hello World"));
        let slice: Cow<'_, str> = pi.0.get_slice(0..5);
        assert_eq!(slice, "Hello");
        assert!(!is_cow_owned(&slice));
    }

    #[test]
    fn cow_owned_from_cow_owned<'s>() {
        let pi: PatternItem<Cow<'s, str>> = PatternItem(Cow::Owned(String::from("Hello World")));
        let slice: Cow<'_, str> = pi.0.get_slice(0..5);
        assert_eq!(slice, "Hello");
        assert!(is_cow_owned(&slice));
    }
}

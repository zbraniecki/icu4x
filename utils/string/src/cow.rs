use crate::Slice;
use std::{borrow::Cow, ops::Range};

impl Slice for Cow<'_, str> {
    fn get_byte(&self, idx: usize) -> Option<u8> {
        self.as_bytes().get(idx).copied()
    }

    fn get_slice(&self, range: Range<usize>) -> Self {
        match self {
            Self::Borrowed(b) => Cow::Borrowed(&b[range]),
            Self::Owned(o) => Cow::Owned(o[range].to_string()),
        }
    }

    fn get_str_slice<'p>(&'p self, range: Range<usize>) -> &'p str {
        &self[range]
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
    fn cow_owned_from_cow_owned<'p>() {
        let pi: PatternItem<Cow<'p, str>> = PatternItem(Cow::Owned(String::from("Hello World")));
        let slice: Cow<'_, str> = pi.0.get_slice(0..5);
        assert_eq!(slice, "Hello");
        assert!(is_cow_owned(&slice));
    }
}

use crate::Slice;
use std::{borrow::Cow, ops::Range};

impl<'s> Slice<'s, String> for String {
    fn get_byte(&self, idx: usize) -> Option<u8> {
        self.as_bytes().get(idx).copied()
    }

    fn get_str_slice(&self, range: Range<usize>) -> &str {
        &self[range]
    }

    fn get_cow_slice(&self, range: Range<usize>) -> Cow<'s, str> {
        Cow::Owned(self[range].to_string())
    }

    fn get_slice(&'s self, range: Range<usize>) -> String {
        self[range].to_string()
    }

    fn length(&self) -> usize {
        self.len()
    }
}

impl<'s> Slice<'s, Cow<'s, str>> for String {
    fn get_byte(&self, idx: usize) -> Option<u8> {
        self.as_bytes().get(idx).copied()
    }

    fn get_str_slice(&self, range: Range<usize>) -> &str {
        &self[range]
    }

    fn get_cow_slice(&self, range: Range<usize>) -> Cow<'s, str> {
        Cow::Owned(self[range].to_string())
    }

    fn get_slice(&'s self, range: Range<usize>) -> Cow<'s, str> {
        Cow::Borrowed(&self[range])
    }

    fn length(&self) -> usize {
        self.len()
    }
}

impl<'s> Slice<'s, &'s str> for String {
    fn get_byte(&self, idx: usize) -> Option<u8> {
        self.as_bytes().get(idx).copied()
    }

    fn get_str_slice(&self, range: Range<usize>) -> &str {
        &self[range]
    }

    fn get_cow_slice(&self, range: Range<usize>) -> Cow<'s, str> {
        Cow::Owned(self[range].to_string())
    }

    fn get_slice(&'s self, range: Range<usize>) -> &'s str {
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

    #[test]
    fn string_from_string<'s>() {
        let pi = PatternItem(String::from("Hello World"));
        let slice: String = pi.0.get_slice(0..5);
        assert_eq!(slice, String::from("Hello"));
    }
}

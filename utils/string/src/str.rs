use crate::Slice;
use std::borrow::Cow;
use std::ops::Range;

impl<'s> Slice<'s, &'s str> for &'s str {
    fn get_byte(&self, idx: usize) -> Option<u8> {
        self.as_bytes().get(idx).copied()
    }

    fn get_str_slice(&self, range: Range<usize>) -> &str {
        &self[range]
    }

    fn get_cow_slice(&self, range: Range<usize>) -> Cow<'s, str> {
        Cow::Borrowed(&self[range])
    }

    fn get_slice(&'s self, range: Range<usize>) -> &'s str {
        &self[range]
    }

    fn length(&self) -> usize {
        self.len()
    }
}

impl<'s> Slice<'s, String> for &'s str {
    fn get_byte(&self, idx: usize) -> Option<u8> {
        self.as_bytes().get(idx).copied()
    }

    fn get_str_slice(&self, range: Range<usize>) -> &str {
        &self[range]
    }

    fn get_cow_slice(&self, range: Range<usize>) -> Cow<'s, str> {
        Cow::Borrowed(&self[range])
    }

    fn get_slice(&'s self, range: Range<usize>) -> String {
        self[range].to_string()
    }

    fn length(&self) -> usize {
        self.len()
    }
}

impl<'s> Slice<'s, Cow<'s, str>> for &'s str {
    fn get_byte(&self, idx: usize) -> Option<u8> {
        self.as_bytes().get(idx).copied()
    }

    fn get_str_slice(&self, range: Range<usize>) -> &str {
        &self[range]
    }

    fn get_cow_slice(&self, range: Range<usize>) -> Cow<'s, str> {
        Cow::Borrowed(&self[range])
    }

    fn get_slice(&'s self, range: Range<usize>) -> Cow<'s, str> {
        Cow::Borrowed(&self[range])
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
    fn str_from_str() {
        let pi = PatternItem("Hello World");
        let slice: &str = pi.0.get_slice(0..5);
        assert_eq!(slice, "Hello");
    }
}

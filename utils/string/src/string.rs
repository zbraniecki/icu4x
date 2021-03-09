use crate::Slice;
use std::ops::Range;

impl Slice for String {
    fn get_byte(&self, idx: usize) -> Option<u8> {
        self.as_bytes().get(idx).copied()
    }

    fn get_slice(&self, range: Range<usize>) -> Self {
        self[range].to_string()
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

    #[test]
    fn string_from_string<'p>() {
        let pi = PatternItem(String::from("Hello World"));
        let slice: String = pi.0.get_slice(0..5);
        assert_eq!(slice, String::from("Hello"));
    }
}

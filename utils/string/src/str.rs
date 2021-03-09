use crate::Slice;
use std::ops::Range;

impl Slice for &str {
    fn get_byte(&self, idx: usize) -> Option<u8> {
        self.as_bytes().get(idx).copied()
    }

    fn get_slice<'p>(&self, range: Range<usize>) -> Self {
        &self[range]
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
    fn str_from_str() {
        let pi = PatternItem("Hello World");
        let slice = pi.0.get_slice(0..5);
        assert_eq!(slice, "Hello");
    }
}

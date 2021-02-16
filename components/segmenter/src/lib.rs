// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! `icu_segmenter` is one of the [`ICU4X`] components.
//!
//! This API provides necessary functionality for text boundary analysis and segmentation.
mod errors;
pub mod options;

pub use errors::SegmenterError;
pub use icu_locid::LanguageIdentifier;
pub use options::SegmenterOptions;
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum Segment {
    Line(Range<usize>),
}

pub struct Segmenter {
    pub locale: LanguageIdentifier,
    pub options: SegmenterOptions,
}

impl Segmenter {
    pub fn try_new(
        locale: LanguageIdentifier,
        options: SegmenterOptions,
    ) -> Result<Self, SegmenterError> {
        Ok(Self { locale, options })
    }

    pub fn segment<'s>(&self, input: &'s str) -> impl Iterator<Item = Segment> + 's {
        let mut lines = input.lines();
        let mut line_start = 0;
        std::iter::from_fn(move || {
            while let Some(line) = lines.next() {
                let range = line_start..line_start + line.len();
                line_start += line.len() + 1;
                return Some(Segment::Line(range));
            }
            None
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let segmenter = Segmenter::try_new(
            LanguageIdentifier::default(),
            SegmenterOptions {
                granularity: options::SegmenterGranularity::Line,
            },
        )
        .expect("Failed to construct a segmenter");
        let mut segments = segmenter.segment("Hello\nWorld");
        assert_eq!(segments.next(), Some(Segment::Line(0..5)));
        assert_eq!(segments.next(), Some(Segment::Line(6..11)));
        assert_eq!(segments.next(), None);
    }
}

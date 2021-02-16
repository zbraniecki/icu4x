// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

pub enum SegmenterGranularity {
    Line,
}

impl Default for SegmenterGranularity {
    fn default() -> Self {
        Self::Line
    }
}

#[derive(Default)]
pub struct SegmenterOptions {
    pub granularity: SegmenterGranularity,
}

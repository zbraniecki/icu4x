// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum SegmenterError {
    Generic,
}

impl Display for SegmenterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Generic => write!(f, "Generic error"),
        }
    }
}

impl Error for SegmenterError {}

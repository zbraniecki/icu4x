mod cow;
mod str;
mod string;

use std::ops::Range;

pub trait Slice {
    fn get_byte(&self, idx: usize) -> Option<u8>;
    fn get_slice(&self, range: Range<usize>) -> Self;
    fn get_str_slice<'p>(&'p self, _range: Range<usize>) -> &'p str;
    fn length(&self) -> usize;
}

pub use crate::cow::*;
pub use crate::str::*;
pub use crate::string::*;

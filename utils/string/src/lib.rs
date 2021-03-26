mod cow;
mod str;
mod string;

use std::borrow::Cow;
use std::ops::Range;

pub trait Slice<'s, RS> where RS: ?Sized {
    fn get_byte(&self, idx: usize) -> Option<u8>;
    fn get_str_slice(&self, _range: Range<usize>) -> &str;
    fn get_cow_slice(&self, _range: Range<usize>) -> Cow<'s, str>;
    fn get_slice(&'s self, range: Range<usize>) -> RS;
    fn length(&self) -> usize;
}

pub use crate::cow::*;
pub use crate::str::*;
pub use crate::string::*;

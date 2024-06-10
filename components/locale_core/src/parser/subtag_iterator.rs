use crate::subtags::Subtag;

#[inline]
const fn is_separator(slice_ptr: *const u8, idx: usize, elem_len: usize) -> bool {
    let byte = unsafe { *slice_ptr.add(idx * elem_len) };
    byte == b'-' || byte == b'_'
}

const fn get_current_subtag(
    slice_ptr: *const u8,
    slice_len: usize,
    idx: usize,
    elem_len: usize,
) -> (usize, usize) {
    debug_assert!(idx < slice_len);

    // This function is called only on the idx == 0 or on a separator.
    let (start, mut end) = if is_separator(slice_ptr, idx, elem_len) {
        // If it's a separator, set the start to idx+1 and advance the idx to the next char.
        (idx + 1, idx + 1)
    } else {
        // If it's idx=0, start is 0 and end is set to 1
        debug_assert!(idx == 0);
        (0, 1)
    };

    while end < slice_len && !is_separator(slice_ptr, end, elem_len) {
        // Advance until we reach end of slice or a separator.
        end += 1;
    }
    // Notice: this slice may be empty (start == end) for cases like `"en-"` or `"en--US"`
    (start, end)
}
#[derive(Copy, Clone, Debug)]
struct SubtagIteratorInner {
    slice_ptr: *const u8,
    slice_len: usize,
    elem_len: usize,
    subtag: (usize, usize),
    done: bool,
}

impl SubtagIteratorInner {
    pub const fn new(slice_ptr: *const u8, slice_len: usize, elem_len: usize) -> Self {
        assert!(
            elem_len == 1 || elem_len == 2,
            "Element length must be 8 or 16 bits"
        );
        let subtag = if slice_len == 0 || is_separator(slice_ptr, 0, elem_len) {
            // This returns (0, 0) which returns Some(b"") for slices like `"-en"` or `"-"`
            (0, 0)
        } else {
            get_current_subtag(slice_ptr, slice_len, 0, elem_len)
        };

        Self {
            slice_ptr,
            slice_len,
            elem_len,
            subtag,
            done: false,
        }
    }

    pub const fn next_manual(mut self) -> (Self, Option<(usize, usize)>) {
        if self.done {
            return (self, None);
        }
        let result = self.subtag;
        if result.1 < self.slice_len {
            self.subtag =
                get_current_subtag(self.slice_ptr, self.slice_len, result.1, self.elem_len);
        } else {
            self.done = true;
        }
        (self, Some(result))
    }

    pub const fn peek_manual(&self) -> Option<(usize, usize)> {
        if self.done {
            return None;
        }
        Some(self.subtag)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SubtagIterator<'a, T> {
    pub slice: &'a [T],
    inner: SubtagIteratorInner,
}

impl<'a, T> SubtagIterator<'a, T> {
    pub const fn new(slice: &'a [T]) -> Self {
        let (slice_ptr, slice_len, elem_len) = if core::mem::size_of::<T>() == 1 {
            (slice.as_ptr() as *const u8, slice.len(), 1)
        } else {
            (slice.as_ptr() as *const u8, slice.len(), 2)
        };

        Self {
            slice,
            inner: SubtagIteratorInner::new(slice_ptr, slice_len, elem_len),
        }
    }

    pub const fn next_manual(mut self) -> (Self, Option<(usize, usize)>)
    where
        T: Copy,
    {
        let (inner, result) = self.inner.next_manual();
        self.inner = inner;
        (self, result)
    }

    pub const fn peek_manual(&self) -> Option<(usize, usize)> {
        self.inner.peek_manual()
    }

    pub fn peek(&self) -> Option<&'a [T]> {
        #[allow(clippy::indexing_slicing)] // peek_manual returns valid indices
        self.peek_manual().map(|(s, e)| &self.slice[s..e])
    }
}

impl<'a, T> Iterator for SubtagIterator<'a, T>
where
    T: Copy,
{
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        let (s, res) = self.next_manual();
        *self = s;
        #[allow(clippy::indexing_slicing)] // next_manual returns valid indices
        res.map(|(s, e)| &self.slice[s..e])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let sample_utf8: Vec<u8> = String::from("foo-bar-baz").into_bytes();
        let sample_utf16: Vec<u16> = String::from("foo-bar-baz").encode_utf16().collect();

        let utf8_ptr = sample_utf8.as_ptr();
        let utf8_len = sample_utf8.len();

        let utf16_ptr = sample_utf16.as_ptr() as *const u8;
        let utf16_len = sample_utf16.len();

        let iter = SubtagIteratorInner::new(utf8_ptr, utf8_len, 1);
        let (iter, value) = iter.next_manual();
        assert_eq!(value, Some((0, 3)));
        let (iter, value) = iter.next_manual();
        assert_eq!(value, Some((4, 7)));
        let (iter, value) = iter.next_manual();
        assert_eq!(value, Some((8, 11)));
        let (iter, value) = iter.next_manual();
        assert_eq!(value, None);

        let iter = SubtagIteratorInner::new(utf16_ptr, utf16_len, 2);
        let (iter, value) = iter.next_manual();
        assert_eq!(value, Some((0, 3)));
        let (iter, value) = iter.next_manual();
        assert_eq!(value, Some((4, 7)));
        let (iter, value) = iter.next_manual();
        assert_eq!(value, Some((8, 11)));
        let (iter, value) = iter.next_manual();
        assert_eq!(value, None);

        let iter = SubtagIterator::new(&sample_utf8);
        let (iter, value) = iter.next_manual();
        assert_eq!(value, Some((0, 3)));
        let (iter, value) = iter.next_manual();
        assert_eq!(value, Some((4, 7)));
        let (iter, value) = iter.next_manual();
        assert_eq!(value, Some((8, 11)));
        let (iter, value) = iter.next_manual();
        assert_eq!(value, None);

        let iter = SubtagIterator::new(&sample_utf16);
        let (iter, value) = iter.next_manual();
        assert_eq!(value, Some((0, 3)));
        let (iter, value) = iter.next_manual();
        assert_eq!(value, Some((4, 7)));
        let (iter, value) = iter.next_manual();
        assert_eq!(value, Some((8, 11)));
        let (iter, value) = iter.next_manual();
        assert_eq!(value, None);

        let iter = SubtagIterator::new(&sample_utf8);
        assert_eq!(iter.collect::<Vec<_>>(), vec![b"foo", b"bar", b"baz"]);

        let iter = SubtagIterator::new(&sample_utf16);
        assert_eq!(
            iter.collect::<Vec<_>>(),
            vec![
                &String::from("foo").encode_utf16().collect::<Vec<_>>(),
                &String::from("bar").encode_utf16().collect::<Vec<_>>(),
                &String::from("baz").encode_utf16().collect::<Vec<_>>(),
            ]
        );
    }
}

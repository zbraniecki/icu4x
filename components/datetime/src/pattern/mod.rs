use zerovec::{
    ule::{AsULE, ULE},
    ZeroVec,
};

struct PatternItemTryFromError;

#[derive(Copy, Clone)]
struct EncodedPatternItem([u8; 3]);

impl ULE for EncodedPatternItem {
    type Error = PatternItemTryFromError;

    fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error> {
        todo!()
    }

    fn as_byte_slice(slice: &[Self]) -> &[u8] {
        todo!()
    }
}

// #[derive(Copy, Clone)]
// enum PatternItem<K, V>
// where
//     K: Copy + ULE,
//     V: Copy + ULE,
// {
//     Field(K, V),
//     Char(char),
// }

// impl<K, V> AsULE for PatternItem<K, V>
// where
//     K: Copy + ULE,
//     V: Copy + ULE,
// {
//     type ULE = EncodedPatternItem;
// }

// struct Pattern<'data, K, V> {
//     items: ZeroVec<'data, PatternItem<K, V>>,
// }

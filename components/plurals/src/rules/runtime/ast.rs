// This file is part of ICU4X condition: (), samples: ()  condi conjunction: (), operand: (), modulo: (), equal: (), range_list: ()  conjunction: (), operand: (), modulo: (), equal: (), range_list: ()  conjunction: (), operand: (), modulo: (), equal: (), range_list: ()  conjunction: (), operand: (), modulo: (), equal: (), range_list: ()  conjunction: (), operand: (), modulo: (), equal: (), range_list: () tion: (), samples: () . For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerovec::ule::{AsULE, AsVarULE, PlainOldULE, VarULE};
use zerovec::VarZeroVec;
use zerovec::ZeroVec;

#[derive(Clone, Debug, PartialEq)]
pub struct Rule<'data>(pub VarZeroVec<'data, Relation<'data>>);

#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    N,
    I,
    V,
    W,
    F,
    T,
    C,
    E,
}

#[derive(Debug, PartialEq)]
pub struct ULERelation([u8]);

unsafe impl VarULE for ULERelation {
    type Error = ();

    #[inline]
    fn parse_byte_slice(bytes: &[u8]) -> Result<&Self, Self::Error> {
        todo!()
        //XXX: Validate
        // Self(bytes)
    }

    #[inline]
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        todo!()
        // Self(bytes)
    }

    #[inline]
    fn as_byte_slice(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Relation<'data> {
    conjunction: bool,
    operand: Operand,
    modulo: u32,
    equal: bool,
    range_list: ZeroVec<'data, RangeOrValue>,
}

impl<'data> AsVarULE for Relation<'data> {
    type VarULE = ULERelation;

    #[inline]
    fn as_unaligned(&self) -> &ULERelation {
        // <--- How to return refernece?
        todo!()
        // let bytes = [
        //   self.conjunction as u8,
        //   self.equal as u8,
        //   **self.range_list.as_unaligned(), // <---- how to build array?
        // ];
        // ULERelation(bytes)
    }
    #[inline]
    fn from_unaligned(unaligned: &ULERelation) -> Self {
        todo!()
        // let conjunction = unaligned[0];
        // let equal = unaligned[1];
        // let range_list = ZeroVec::from_unaligned(&unaligned[2..]);
        // Self {
        //     conjunction,
        //     equal,
        //     range_list
        // }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RangeOrValue {
    Range(u32, u32),
    Value(u32),
}

impl AsULE for RangeOrValue {
    type ULE = PlainOldULE<8>;

    #[inline]
    fn as_unaligned(&self) -> Self::ULE {
        let mut result = [0; 8];
        match self {
            Self::Range(start, end) => {
                let start_bytes = start.to_le_bytes();
                let end_bytes = end.to_le_bytes();
                result[..4].copy_from_slice(&start_bytes);
                result[4..].copy_from_slice(&end_bytes);
                result.into()
            }
            Self::Value(idx) => {
                let bytes = idx.to_le_bytes();
                result[..4].copy_from_slice(&bytes);
                result[4..].copy_from_slice(&bytes);
                result.into()
            }
        }
    }

    #[inline]
    fn from_unaligned(unaligned: &Self::ULE) -> Self {
        let b = unaligned.0;
        let start = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
        let end = u32::from_le_bytes([b[4], b[5], b[6], b[7]]);
        if start == end {
            Self::Value(start)
        } else {
            Self::Range(start, end)
        }
    }
}

impl From<&crate::rules::ast::Rule> for Rule<'_> {
    fn from(input: &crate::rules::ast::Rule) -> Self {
        todo!()
    }
}

mod test {
    use super::*;
    use crate::rules::ast;
    use crate::rules::parse;

    #[test]
    fn sanity_test() {
        // let input = "n % 10 = 3..4,9 and n % 100 != 10..19,70..79,90..99 or n = 0";
        let input = "n = 1";
        let full_ast = parse(input.as_bytes()).unwrap();
        assert_eq!(
            full_ast,
            ast::Rule {
                condition: ast::Condition(Box::new([ast::AndCondition(Box::new([
                    ast::Relation {
                        expression: ast::Expression {
                            operand: ast::Operand::N,
                            modulus: None,
                        },
                        operator: ast::Operator::Eq,
                        range_list: ast::RangeList(Box::new([ast::RangeListItem::Value(
                            ast::Value(1)
                        )]))
                    }
                ]))])),
                samples: None,
            }
        );

        let runtime_ast = Rule::from(&full_ast);
        assert_eq!(
            runtime_ast,
            Rule(VarZeroVec::from(vec![Relation {
                conjunction: true,
                operand: Operand::N,
                modulo: 0,
                equal: true,
                range_list: ZeroVec::clone_from_slice(&[RangeOrValue::Value(1)])
            }]))
        );
    }
}

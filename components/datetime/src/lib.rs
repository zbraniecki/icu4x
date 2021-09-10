pub mod fields;
#[cfg(feature = "provider_serde")]
pub mod fixtures;
pub mod pattern;

#[cfg(test)]
mod test {
    use super::*;
    use pattern::{Pattern, PatternItem, ZVPattern};
    use zerovec::{
        ule::{AsULE, ULE},
        ZeroVec,
    };

    #[test]
    fn test_lengths() {
        let data = (
            &[0x01, 0x02, 0x06],
            &[
                fields::FieldLength::One,
                fields::FieldLength::TwoDigit,
                fields::FieldLength::Six,
            ],
        );
        let lengths = fields::FieldLength::parse_byte_slice(data.0).unwrap();
        assert_eq!(lengths.len(), data.1.len());
        for i in 0..lengths.len() {
            assert_eq!(lengths.get(i), Some(&data.1[i]));
        }
        let zv = ZeroVec::try_from_bytes(data.0).unwrap();
        assert_eq!(zv.len(), data.1.len());
        for i in 0..zv.len() {
            assert_eq!(zv.get(i), Some(data.1[i]));
        }
        assert_eq!(zv.as_bytes(), data.0);
    }

    #[test]
    fn test_years() {
        let data = (
            &[0x00, 0x01],
            &[fields::Year::Calendar, fields::Year::WeekOf],
        );
        let years = fields::Year::parse_byte_slice(data.0).unwrap();
        assert_eq!(years.len(), data.1.len());
        for i in 0..years.len() {
            assert_eq!(years.get(i), Some(&data.1[i]));
        }
        let zv = ZeroVec::try_from_bytes(data.0).unwrap();
        assert_eq!(zv.len(), data.1.len());
        for i in 0..zv.len() {
            assert_eq!(zv.get(i), Some(data.1[i]));
        }
        assert_eq!(zv.as_bytes(), data.0);
    }

    #[test]
    fn test_symbols() {
        let data = (
            &[0x01, 0x00, 0x00, 0x01],
            &[
                fields::FieldSymbol::Month(fields::Month::Short),
                fields::FieldSymbol::Year(fields::Year::WeekOf),
            ],
        );
        let symbols = fields::FieldSymbol::parse_byte_slice(data.0).unwrap();
        assert_eq!(symbols.len(), data.1.len());
        for i in 0..symbols.len() {
            assert_eq!(symbols.get(i), Some(&data.1[i]));
        }
        let zv = ZeroVec::try_from_bytes(data.0).unwrap();
        assert_eq!(zv.len(), data.1.len());
        for i in 0..zv.len() {
            assert_eq!(zv.get(i), Some(data.1[i]));
        }
        assert_eq!(zv.as_bytes(), data.0);
    }

    #[test]
    fn test_fields() {
        let data = (
            &[0x00, 0x00, 0x01, 0x01, 0x00, 0x02],
            &[
                fields::Field {
                    symbol: fields::FieldSymbol::Year(fields::Year::Calendar),
                    length: fields::FieldLength::One,
                },
                fields::Field {
                    symbol: fields::FieldSymbol::Month(fields::Month::Short),
                    length: fields::FieldLength::TwoDigit,
                },
            ],
        );
        let fields = fields::Field::parse_byte_slice(data.0).unwrap();
        assert_eq!(fields.len(), data.1.len());
        for i in 0..fields.len() {
            assert_eq!(fields.get(i), Some(&data.1[i]));
        }
        let zv = ZeroVec::try_from_bytes(data.0).unwrap();
        assert_eq!(zv.len(), data.1.len());
        for i in 0..zv.len() {
            assert_eq!(zv.get(i), Some(data.1[i]));
        }
        assert_eq!(zv.as_bytes(), data.0);
    }

    #[test]
    fn test_pattern_items() {
        let data = (
            &[
                0b0000_0000,
                0b0000_0000,
                0b0000_0001,
                0b0000_0001,
                0b0000_0001,
                0b0000_0010,
                0b1000_0000,
                0b0000_0000,
                0b0110_0001,
            ],
            &[
                pattern::PatternItem::Field(fields::Field {
                    symbol: fields::FieldSymbol::Year(fields::Year::Calendar),
                    length: fields::FieldLength::One,
                }),
                pattern::PatternItem::Field(fields::Field {
                    symbol: fields::FieldSymbol::Month(fields::Month::Short),
                    length: fields::FieldLength::TwoDigit,
                }),
                pattern::PatternItem::Literal('a'),
            ],
        );
        let encoded_items = pattern::EncodedPatternItem::parse_byte_slice(data.0).unwrap();
        let items = encoded_items
            .iter()
            .map(|i| pattern::PatternItem::from_unaligned(i))
            .collect::<Vec<_>>();
        assert_eq!(items.len(), data.1.len());
        for i in 0..items.len() {
            assert_eq!(items.get(i), Some(&data.1[i]));
        }
        let zv = ZeroVec::try_from_bytes(data.0).unwrap();
        assert_eq!(zv.len(), data.1.len());
        for i in 0..zv.len() {
            assert_eq!(zv.get(i), Some(data.1[i]));
        }
        assert_eq!(zv.as_bytes(), data.0);
    }

    #[test]
    fn test_field_symbol_from_u8() {
        let data = (
            &[0b0000_0000, 0b0000_0001, 0b0001_0000],
            &[
                fields::FieldSymbol::Year(fields::Year::Calendar),
                fields::FieldSymbol::Month(fields::Month::Short),
                fields::FieldSymbol::Year(fields::Year::WeekOf),
            ],
        );
        for i in 0..data.0.len() {
            let fs = fields::FieldSymbol::from(data.0[i]);
            assert_eq!(fs, data.1[i]);

            let bytes = u8::from(fs);
            assert_eq!(data.0[i], bytes);
        }
    }

    #[test]
    fn test_pattern_from_bytes() {
        let data = (
            &[
                0b0000_0000,
                0b0000_0000,
                0b0000_0001,
                0b0000_0000,
                0b0000_0001,
                0b0000_0010,
                0b1000_0000,
                0b0000_0000,
                0b0110_0001,
            ],
            &[
                PatternItem::Field(fields::Field {
                    symbol: fields::FieldSymbol::Year(fields::Year::Calendar),
                    length: fields::FieldLength::One,
                }),
                PatternItem::Field(fields::Field {
                    symbol: fields::FieldSymbol::Month(fields::Month::Short),
                    length: fields::FieldLength::TwoDigit,
                }),
                PatternItem::Literal('a'),
            ],
        );
        let pattern = Pattern(data.1.to_vec());
        let zv_pattern = ZVPattern(ZeroVec::try_from_bytes(data.0).unwrap());
        assert_eq!(zv_pattern.0.len(), data.1.len());
        for i in 0..pattern.0.len() {
            assert_eq!(zv_pattern.0.get(i).as_ref(), pattern.0.get(i));
        }
        assert_eq!(zv_pattern.0.as_bytes(), data.0);

        let zv_pattern: ZVPattern = (&pattern).into();
        assert_eq!(zv_pattern.0.len(), data.1.len());
        for i in 0..pattern.0.len() {
            assert_eq!(zv_pattern.0.get(i).as_ref(), pattern.0.get(i));
        }
        assert_eq!(zv_pattern.0.as_bytes(), data.0);
    }
}

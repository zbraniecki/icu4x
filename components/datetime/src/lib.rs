mod fields;
mod pattern;
use zerovec::ule::ULE;
use zerovec::ZeroVec;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lengths() {
        let data = (
            &[0x01, 0x02, 0x06],
            &[
                fields::FieldLength::One,
                fields::FieldLength::TwoDigit,
                fields::FieldLength::Six,
            ]
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
    }

    #[test]
    fn test_years() {
        let data = (
            &[0x00, 0x01],
            &[
                fields::Year::Calendar,
                fields::Year::WeekOf,
            ]
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
    }

    #[test]
    fn test_symbols() {
        let data = (
            &[0x01, 0x00, 0x00, 0x01],
            &[
                fields::FieldSymbol::Month(fields::Month::Short),
                fields::FieldSymbol::Year(fields::Year::WeekOf),
            ]
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
            ]
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
    }

    #[test]
    fn test_pattern_items() {
        let data = (
            &[0x00, 0x00, 0x01],
            &[
                pattern::PatternItem::Field(
                    fields::Field {
                        symbol: fields::FieldSymbol::Year(fields::Year::Calendar),
                        length: fields::FieldLength::One,
                    }
                ),
            ]
        );
        let items = pattern::PatternItem::parse_byte_slice(data.0).unwrap();
        assert_eq!(items.len(), data.1.len());
        println!("{:#?}", items);
        // for i in 0..lengths.len() {
        //     assert_eq!(lengths.get(i), Some(&data.1[i]));
        // }
        // let zv = ZeroVec::try_from_bytes(data.0).unwrap();
        // assert_eq!(zv.len(), data.1.len());
        // for i in 0..zv.len() {
        //     assert_eq!(zv.get(i), Some(data.1[i]));
        // }
    }
}

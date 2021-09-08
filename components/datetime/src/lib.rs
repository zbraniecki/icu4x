mod fields;
mod pattern;
use zerovec::ule::ULE;
use zerovec::ZeroVec;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn internal() {
        // let len = fields::FieldLength::One;

        // let bytes: &[u8] = &[0x01, 0x02, 0x06];
        // let lengths = fields::FieldLength::parse_byte_slice(bytes).unwrap();
        // println!("{:#?}", lengths);
        // let zv: ZeroVec<fields::FieldLength> = ZeroVec::try_from_bytes(bytes).unwrap();
        // println!("{:#?}", zv);

        // let bytes = &[0x00, 0x01];
        // let years = fields::Year::parse_byte_slice(bytes).unwrap();
        // println!("{:#?}", years);
        // let zv: ZeroVec<fields::Year> = ZeroVec::try_from_bytes(bytes).unwrap();
        // println!("{:#?}", zv);

        // let bytes = &[0x01, 0x00, 0x00, 0x01];
        // let symbols = fields::FieldSymbol::parse_byte_slice(bytes).unwrap();
        // println!("{:#?}", symbols);
        // let zv: ZeroVec<fields::FieldSymbol> = ZeroVec::try_from_bytes(bytes).unwrap();
        // println!("{:#?}", zv);

        let bytes = &[0x00, 0x00, 0x01, 0x01, 0x00, 0x02];
        let fields = fields::Field::parse_byte_slice(bytes).unwrap();
        println!("{:#?}", fields);
        let zv: ZeroVec<fields::Field> = ZeroVec::try_from_bytes(bytes).unwrap();
        println!("{:#?}", zv);
    }
}

use crate::{regex::{regex_map, REG_UNIT, REG_UNIT_ABBVR, REG_UNIT_CHAR, REG_UNIT_NO}, Address};

macro_rules! unit_map {
    ($address:ident) => {{
        let mut unit_no = regex_map(&$address.full_address, &REG_UNIT_NO);
        if unit_no.is_none(){
            unit_no = regex_map(&$address.full_address, &REG_UNIT_CHAR);
        } 
        $address.unit_no = unit_no;
        return $address
    }};
}

pub fn parse_unit_type_and_no(mut address: Address) -> Address {
    address.unit_type = regex_map(&address.full_address, &REG_UNIT_ABBVR);
    if address.unit_type.is_none() {
        address.unit_type = regex_map(&address.full_address, &REG_UNIT);
        if address.unit_type.is_some() {
            unit_map!(address)
        } else { 
            address.unit_no = None;
            address
        }
    } else {
        unit_map!(address)
    }
}
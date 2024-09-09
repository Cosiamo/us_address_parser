use crate::{components::Unit, regex::{regex_map, REG_UNIT, REG_UNIT_ABBVR, REG_UNIT_CHAR, REG_UNIT_NO}, regex_map_string, Address};

macro_rules! unit_map {
    ($address:ident, $unit_type:ident, $full_address:ident, $unit:ident) => {{
        let mut unit_no = regex_map_string!($full_address, REG_UNIT_NO);
        if unit_no.len() == 0 {
            unit_no = regex_map_string!($full_address, REG_UNIT_CHAR);
        } 
        $address.unit_no = unit_no;
        $address.unit_type = Unit::$unit_type($unit);
        return $address
    }};
}

pub fn parse_unit_type_and_no(mut address: Address, full_address: &str) -> Address {
    let unit_abbvr = regex_map_string!(full_address, REG_UNIT_ABBVR);
    if unit_abbvr.len() > 0 {
        unit_map!(address, Abbreviated, full_address, unit_abbvr)
    }

    let unit = regex_map_string!(full_address, REG_UNIT);
    if unit.len() > 0 {
        unit_map!(address, Full, full_address, unit)
    }

    address
}
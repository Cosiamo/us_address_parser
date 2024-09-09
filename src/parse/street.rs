use crate::{regex::{regex_map, REG_STREET_NUMBER}, regex_map_string, Address};

pub fn parse_street_no(mut address: Address, full_address: &str) -> Address {
    address.street_no = regex_map_string!(full_address, REG_STREET_NUMBER);
    address
}

pub fn parse_street_name(mut address: Address, full_address: &str) -> Address {
    let street_no = &address.street_no;
    let direction = &address.direction.to_string();
    let street_type = &address.street_type.to_string();
    let unit_type = &address.unit_type.to_string();
    let unit_no = &address.unit_no;

    let street_name = format!(" {} ", full_address)
        .replace(&format!(" {} ", street_type), " ")
        .replace(street_no, "")
        .replace(&format!(" {} ", direction), " ")
        .replace(&format!(" {} ", unit_type), " ")
        .replace(&format!(" {} ", unit_no), "");

    address.street_name = street_name.trim().replace("\"", "");
    address
}
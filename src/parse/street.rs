use crate::{regex::{regex_map, REG_STREET_NUMBER}, Address};

pub fn parse_street_no(mut address: Address, full_address: &str) -> Address {
    address.street_no = regex_map(full_address, &REG_STREET_NUMBER);
    address
}

pub fn parse_street_name(mut address: Address, full_address: &str) -> Address {
    let mut street_name = format!(" {} ", full_address);

    match &address.street_no {
        Some(val) => street_name = street_name.replace(val, ""),
        None => street_name = street_name,
    }

    match &address.direction {
        Some(val) => street_name = street_name.replace(val, ""),
        None => street_name = street_name,
    }

    match &address.street_type {
        Some(val) => street_name = street_name.replace(&format!(" {} ", val), " "),
        None => street_name = street_name,
    }

    match &address.unit_type {
        Some(val) => {
            street_name = street_name.replace(&format!(" {} ", val), " ");
            match &address.unit_no {
                Some(val) => street_name = street_name.replace(&format!(" {} ", val), " "),
                None => street_name = street_name,
            }
        },
        None => street_name = street_name,
    }

    address.street_name = Some(street_name.trim().replace("\"", ""));
    address
}
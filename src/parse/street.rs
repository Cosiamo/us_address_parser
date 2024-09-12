use crate::{components::street_type::{STREET_TYPE, STREET_TYPE_ABBVR}, regex::{regex_map, REG_STREET_NUMBER, REG_STRIP_STREET0, REG_STRIP_STREET1, REG_STRIP_STREET2, REG_STRIP_STREET3, REG_STRIP_STREET_ABBVR0, REG_STRIP_STREET_ABBVR1, REG_STRIP_STREET_ABBVR2, REG_STRIP_STREET_ABBVR3}, Address};

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
    
    match &address.street_type {
        Some(val) => {
            let size = STREET_TYPE_ABBVR.len() / 4;
            match val {
                v if STREET_TYPE_ABBVR[..size].contains(&v.to_lowercase().as_str()) => street_name = REG_STRIP_STREET_ABBVR0.replace(&street_name, "").to_string(),
                v if STREET_TYPE_ABBVR[size..size*2].contains(&v.to_lowercase().as_str()) => street_name = REG_STRIP_STREET_ABBVR1.replace(&street_name, "").to_string(),
                v if STREET_TYPE_ABBVR[size*2..size*3].contains(&v.to_lowercase().as_str()) => street_name = REG_STRIP_STREET_ABBVR2.replace(&street_name, "").to_string(),
                v if STREET_TYPE_ABBVR[size*3..].contains(&v.to_lowercase().as_str()) => street_name = REG_STRIP_STREET_ABBVR3.replace(&street_name, "").to_string(),
                val => {
                    match val {
                        v if STREET_TYPE[..size].contains(&v.to_lowercase().as_str()) => street_name = REG_STRIP_STREET0.replace(&street_name, "").to_string(),
                        v if STREET_TYPE[size..size*2].contains(&v.to_lowercase().as_str()) => street_name = REG_STRIP_STREET1.replace(&street_name, "").to_string(),
                        v if STREET_TYPE[size*2..size*3].contains(&v.to_lowercase().as_str()) => street_name = REG_STRIP_STREET2.replace(&street_name, "").to_string(),
                        v if STREET_TYPE[size*3..].contains(&v.to_lowercase().as_str()) => street_name = REG_STRIP_STREET3.replace(&street_name, "").to_string(),
                        _ => street_name = street_name,
                    }
                },
            }
        },
        None => street_name = street_name,
    }

    address.street_name = Some(street_name.trim().replace("\"", ""));
    address
}
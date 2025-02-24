use crate::{regex::{regex_map, REG_STREET_NUMBER}, Address};

pub fn parse_street_no(mut address: Address) -> Address {
    address.street_no = regex_map(&address.full_address, &REG_STREET_NUMBER);
    address
}

pub fn parse_street_name(mut address: Address) -> Address {
    let mut street_name = format!(" {} ", &address.full_address);

    match &address.street_no {
        Some(val) => street_name = street_name.replace(val, ""),
        None => street_name = street_name,
    }

    // adding whitespace to either side of the input address
    // to avoid collateral character strips.
    // ex: N MAIN ST could accidentally have the street name stripped down to 'MAI'
    // if the replace isn't looking for space on BOTH sides of 'N'.
    street_name = format!(" {street_name} ");
    match &address.direction {
        Some(val) => {
            let val = format!(" {val} ");
            street_name = street_name.replace(&val, "")
        },
        None => street_name = street_name,
    }
    street_name = format!(" {street_name} ");
    match &address.unit_type {
        Some(val) => {
            let val = format!(" {val} ");
            street_name = street_name.replace(&val, " ");
            match &address.unit_no {
                Some(val) => street_name = street_name.replace(&format!(" {} ", val), " "),
                None => street_name = street_name,
            }
        },
        None => street_name = street_name,
    }
    street_name = format!(" {street_name} ");
    match &address.street_type {
        Some(val) => street_name = street_name.replace(val, ""),
        None => street_name = street_name,
    }

    address.street_name = Some(street_name.trim().replace("\"", ""));
    address
}
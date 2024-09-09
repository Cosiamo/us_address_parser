use crate::{components::StreetType, regex::{regex_map, REG_STREET0, REG_STREET1, REG_STREET2, REG_STREET3, REG_STREET_ABBVR0, REG_STREET_ABBVR1, REG_STREET_ABBVR2, REG_STREET_ABBVR3}, Address};

macro_rules! get_street_type {
    ($captured:ident, $address:ident, $full_address:ident, $street_type:ident) => {{
        if $captured.len() == 1 {
            $address.street_type = StreetType::$street_type($captured[0].clone());
            return $address
        } else if $captured.len() > 0 {
            let index = find_last_in_haystack(&$captured, $full_address);
            $address.street_type = StreetType::$street_type($captured[index].clone());
            return $address
        }
    }};
}

pub fn parse_street_type(mut address: Address, full_address: &str) -> Address {
    let mut captured: Vec<String> = vec![];
    
    if let Some(val) = regex_map(&full_address, &REG_STREET_ABBVR0) { captured.push(val) };
    if let Some(val) = regex_map(&full_address, &REG_STREET_ABBVR1) { captured.push(val) };
    if let Some(val) = regex_map(&full_address, &REG_STREET_ABBVR2) { captured.push(val) };
    if let Some(val) = regex_map(&full_address, &REG_STREET_ABBVR3) { captured.push(val) };

    get_street_type!(captured, address, full_address, Abbreviated);
    
    if let Some(val) = regex_map(&full_address, &REG_STREET0) { captured.push(val) };
    if let Some(val) = regex_map(&full_address, &REG_STREET1) { captured.push(val) };
    if let Some(val) = regex_map(&full_address, &REG_STREET2) { captured.push(val) };
    if let Some(val) = regex_map(&full_address, &REG_STREET3) { captured.push(val) };

    get_street_type!(captured, address, full_address, Full);
    
    address.street_type = StreetType::Abbreviated("".to_owned());
    return address
}

fn find_last_in_haystack(captured: &Vec<String>, full_address: &str) -> usize {
    let mut index = 0;
    let mut prev_haystack_idx = 0;
    for (idx, needle) in captured.iter().enumerate() {
        let haystack_idx = if let Some(val) = full_address.find(needle) { val } else { 0 };
        if haystack_idx > prev_haystack_idx {
            prev_haystack_idx = haystack_idx;
            index = idx
        }
    }
    index
}
use crate::{regex::{regex_map_street_type, REG_STREET0, REG_STREET1, REG_STREET2, REG_STREET3, REG_STREET_ABBVR0, REG_STREET_ABBVR1, REG_STREET_ABBVR2, REG_STREET_ABBVR3}, Address};

pub fn parse_street_type(mut address: Address, full_address: &str) -> Address {
    let mut captured: Vec<String> = vec![];
    
    if let Some(val) = regex_map_street_type(&full_address, &REG_STREET_ABBVR0) { captured.push(val) };
    if let Some(val) = regex_map_street_type(&full_address, &REG_STREET_ABBVR1) { captured.push(val) };
    if let Some(val) = regex_map_street_type(&full_address, &REG_STREET_ABBVR2) { captured.push(val) };
    if let Some(val) = regex_map_street_type(&full_address, &REG_STREET_ABBVR3) { captured.push(val) };
    
    if let Some(val) = regex_map_street_type(&full_address, &REG_STREET0) { captured.push(val) };
    if let Some(val) = regex_map_street_type(&full_address, &REG_STREET1) { captured.push(val) };
    if let Some(val) = regex_map_street_type(&full_address, &REG_STREET2) { captured.push(val) };
    if let Some(val) = regex_map_street_type(&full_address, &REG_STREET3) { captured.push(val) };

    if captured.len() == 1 {
        address.street_type = Some(captured[0].clone());
        return address
    } else if captured.len() > 0 {
        // If an address is "123 Maple Parks Street" (both street types), 
        // it will only return "Street".
        // Can't use `captured[captured.len() - 1]` because of how the Regex is split up
        let index = find_last_in_haystack(&captured, full_address);
        address.street_type = Some(captured[index].clone());
        return address
    } else {
        address.street_type = None;
        return address
    }
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
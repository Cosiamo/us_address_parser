use crate::{
    Address,
    components::street_type::{STREET_TYPE, STREET_TYPE_ABBVR},
};

pub fn parse_street_type(mut address: Address) -> Address {
    let mut captured = Vec::new();

    let haystack = address.full_address.split(" ").collect::<Vec<&str>>();

    for needle in haystack {
        if STREET_TYPE.contains(&needle.to_ascii_lowercase().as_str()) {
            captured.push(needle.to_string());
        } else if STREET_TYPE_ABBVR.contains(&needle.to_ascii_lowercase().as_str()) {
            captured.push(needle.to_string());
        } else {
            continue;
        }
    }

    if &captured.len() == &1 {
        address.street_type = Some(captured[0].clone());
        return address;
    } else if &captured.len() > &0 {
        // If an address is "123 Maple Parks Street" (both street types),
        // it will only return "Street".
        // Can't use `captured[captured.len() - 1]` because of how the Regex is split up
        let index = find_last_in_haystack(&captured, &address.full_address);
        address.street_type = Some(captured[index].clone());
        return address;
    } else {
        address.street_type = None;
        return address;
    }
}

fn find_last_in_haystack(captured: &Vec<String>, full_address: &str) -> usize {
    let mut index = 0;
    let mut prev_haystack_idx = 0;
    for (idx, needle) in captured.iter().enumerate() {
        let haystack_idx = if let Some(val) = full_address.find(needle) {
            val
        } else {
            0
        };
        if haystack_idx > prev_haystack_idx {
            prev_haystack_idx = haystack_idx;
            index = idx
        } else {
            continue;
        }
    }
    index
}

use crate::{components::Direction, regex::{regex_map, REG_DIRECTIONAL, REG_DIRECTIONAL_ABBVR}, regex_map_string, Address};

pub fn parse_direction(mut address: Address, full_address: &str) -> Address {
    let mut dir = regex_map_string!(full_address, REG_DIRECTIONAL_ABBVR);
    if dir.len() == 0 {
        dir = regex_map_string!(full_address, REG_DIRECTIONAL);
        address.direction = Direction::Full(dir);
        address
    } else {
        address.direction = Direction::Abbreviated(dir);
        address
    }
}
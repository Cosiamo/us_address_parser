use crate::{regex::{regex_map, REG_DIRECTIONAL, REG_DIRECTIONAL_ABBVR}, Address};

pub fn parse_direction(mut address: Address) -> Address {
    let mut dir = regex_map(&address.full_address, &REG_DIRECTIONAL_ABBVR);
    if dir.is_none() {
        dir = regex_map(&address.full_address, &REG_DIRECTIONAL);
        address.direction = dir;
        address
    } else {
        address.direction = dir;
        address
    }
}
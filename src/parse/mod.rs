use direction::parse_direction;
use street::{parse_street_name, parse_street_no};
use street_type::parse_street_type;
use unit::parse_unit_type_and_no;

use crate::Address;

pub mod street_type;
pub mod unit;
pub mod direction;
pub mod street;

impl Address {
    pub fn street_no(self, full_address: &str) -> Self {
        parse_street_no(self, full_address)
    }

    pub fn street_type(self, full_address: &str) -> Self {
        parse_street_type(self, full_address)
    }

    pub fn unit_type_and_no(self, full_address: &str) -> Self {
        parse_unit_type_and_no(self, full_address)
    }

    pub fn directional(self, full_address: &str) -> Self {
        parse_direction(self, full_address)
    }

    pub fn street_name(self, full_address: &str) -> Self {
        parse_street_name(self, full_address)
    }
}